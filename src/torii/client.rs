use async_channel::{unbounded, Receiver};
use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use starknet_crypto::Felt;
use torii_client::client::Client;
use torii_grpc::{
    client::EntityUpdateStreaming,
    types::{schema::Entity as DojoEntity, EntityKeysClause, KeysClause, Query as ToriiQuery},
};

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_torii_client);
        app.add_systems(Update, update_entities);
    }
}

#[derive(Resource)]
pub struct ToriiClient {
    pub entity_rx: Receiver<DojoEntity>,
    pub runtime: tokio::runtime::Runtime,
}

#[derive(Component)]
pub struct BevyEntity {
    pub dojo_entity: DojoEntity,
}

pub fn setup_torii_client(mut commands: Commands) {
    let torii_url = "http://0.0.0.0:8080".to_string();
    let rpc_url = "http://0.0.0.0:5050".to_string();
    let relay_url = "/ip4/127.0.0.1/tcp/9090".to_string();
    let world = Felt::from_hex_unchecked(
        "0x5d97c46d046f442f125b6cc83057e97ee6e848c4921126acd8ae9d17b55b369",
    );
    let (tx, rx) = unbounded();

    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.spawn(async move {
        info!("Setting up Torii client");
        let client = Client::new(torii_url, rpc_url, relay_url, world)
            .await
            .unwrap();
        let mut rcv: EntityUpdateStreaming = client
            .on_entity_updated(vec![EntityKeysClause::Keys(KeysClause {
                keys: vec![],
                pattern_matching: torii_grpc::types::PatternMatching::VariableLen,
                models: vec![],
            })])
            .await
            .unwrap();

        let query = ToriiQuery {
            clause: None,
            limit: 500,
            offset: 0,
        };
        let all_entities = client.entities(query).await.unwrap();
        for entity in all_entities {
            info!("Existing Entities: {:?}", entity);
            // tx.send(entity).await.unwrap();
        }

        info!("Torii client setup");
        while let Some(Ok((_, entity))) = rcv.next().await {
            info!("Received Dojo entity: {:?}", entity);
            tx.send(entity).await.unwrap();
        }
    });

    info!("Torii client setup task spawned");

    commands.insert_resource(ToriiClient {
        entity_rx: rx,
        runtime: runtime,
    });
}

fn update_entities(
    mut commands: Commands,
    client: Res<ToriiClient>,
    mut query: Query<&mut BevyEntity>,
) {
    match client.entity_rx.try_recv() {
        Ok(entity) => {}
        Err(err) => {
            if err != async_channel::TryRecvError::Empty {
                error!("Error receiving entity: {:?}", err);
            }
        }
    }
}
