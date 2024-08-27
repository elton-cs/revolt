use async_channel::{unbounded, Receiver};
use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use starknet_crypto::Felt;
use torii_client::client::Client;
use torii_grpc::{
    client::EntityUpdateStreaming,
    types::{schema::Entity as DojoEntity, EntityKeysClause, KeysClause, Query as ToriiQuery},
};

use crate::utils::constants::{TORII_RELAY_URL, TORII_RPC_URL, TORII_URL, TORII_WORLD_CONTRACT};

use super::tokio::TokioRuntime;

pub struct ToriiPlugin;
impl Plugin for ToriiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_torii_client);
        app.add_systems(Update, update_entities);
    }
}

#[derive(Resource)]
pub struct ToriiClient {
    pub dojo_entity_rx: Receiver<DojoEntity>,
}

#[derive(Component)]
pub struct BevyEntity {
    pub dojo_entity: DojoEntity,
}

pub fn setup_torii_client(mut commands: Commands, tokio: Res<TokioRuntime>) {
    let torii_url = TORII_URL.to_string();
    let rpc_url = TORII_RPC_URL.to_string();
    let relay_url = TORII_RELAY_URL.to_string();
    let world = Felt::from_hex_unchecked(TORII_WORLD_CONTRACT);
    let (tx, rx) = unbounded();

    info!("Torii client setup task spawned");

    tokio.runtime.spawn(async move {
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

    commands.insert_resource(ToriiClient {
        dojo_entity_rx: rx,
        // tokio_runtime: runtime,
    });
}

fn update_entities(
    mut commands: Commands,
    client: Res<ToriiClient>,
    mut query: Query<&mut BevyEntity>,
) {
    match client.dojo_entity_rx.try_recv() {
        Ok(entity) => {}
        Err(err) => {
            if err != async_channel::TryRecvError::Empty {
                error!("Error receiving entity: {:?}", err);
            }
        }
    }
}
