use crate::{
    tokio::TokioRuntime,
    utils::constants::{
        EXISTING_ENTITY_QUERY_LIMIT, TORII_RELAY_URL, TORII_RPC_URL, TORII_URL,
        TORII_WORLD_CONTRACT,
    },
};
use async_channel::{bounded, Receiver};
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
        app.add_systems(Update, spawn_dojo_entities);
        app.add_systems(Update, number_of_dojo_entities);
    }
}

#[derive(Resource)]
pub struct ToriiClient {
    pub dojo_entity_rx: Receiver<DojoEntity>,
}

#[derive(Component)]
pub struct TempDojoEntityWrapper {
    pub dojo_entity: DojoEntity,
}

pub fn setup_torii_client(mut commands: Commands, tokio: Res<TokioRuntime>) {
    let torii_url = TORII_URL.to_string();
    let rpc_url = TORII_RPC_URL.to_string();
    let relay_url = TORII_RELAY_URL.to_string();
    let world = Felt::from_hex_unchecked(TORII_WORLD_CONTRACT);
    let (tx, rx) = bounded(16);

    tokio.runtime.spawn(async move {
        info!("Starting Torii client...");
        let client = Client::new(torii_url, rpc_url, relay_url, world)
            .await
            .unwrap();

        info!("Reading existing entities...");
        let query = ToriiQuery {
            clause: None,
            limit: EXISTING_ENTITY_QUERY_LIMIT,
            offset: 0,
        };
        let all_entities = client.entities(query).await.unwrap();
        for entity in all_entities {
            info!("Existing Entities: {:?}", entity);
            tx.send(entity).await.unwrap();
        }

        info!("Subscribing to entity updates...");
        let mut rcv: EntityUpdateStreaming = client
            .on_entity_updated(vec![EntityKeysClause::Keys(KeysClause {
                keys: vec![],
                pattern_matching: torii_grpc::types::PatternMatching::VariableLen,
                models: vec![],
            })])
            .await
            .unwrap();
        while let Some(Ok((_, entity))) = rcv.next().await {
            info!("Received Dojo entity: {:?}", entity);
            tx.send(entity).await.unwrap();
        }
    });
    info!("Background Torii client setup complete.");

    commands.insert_resource(ToriiClient { dojo_entity_rx: rx });
}

fn spawn_dojo_entities(mut commands: Commands, client: Res<ToriiClient>) {
    match client.dojo_entity_rx.try_recv() {
        Ok(entity) => {
            commands.spawn(TempDojoEntityWrapper {
                dojo_entity: entity,
            });
        }
        Err(err) => {
            if err != async_channel::TryRecvError::Empty {
                error!("Error receiving entity: {:?}", err);
            }
        }
    }
}

fn number_of_dojo_entities(query: Query<&TempDojoEntityWrapper>) {
    let value = query.iter().count();
    info!("Number of Dojo entities: {}", value);
}
