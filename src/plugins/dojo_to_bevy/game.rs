use crate::torii::client::TempDojoEntityWrapper;

use super::type_extractors::{member_to_bool, member_to_contract_address_to_felt, member_to_u32};
use bevy::prelude::*;
use starknet_crypto::Felt;
use torii_grpc::types::schema::Entity as DojoEntity;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_or_update_current_game);
    }
}

#[derive(Component)]
pub struct GameModel {
    pub id: u32,
    pub player_1_address: Felt,
    pub player_2_address: Felt,
    pub player_3_address: Felt,
    pub player_4_address: Felt,
    pub map_id: u32,
    pub state: bool,
    pub winner: Felt,
}

fn spawn_or_update_current_game(
    query_temp_dojo_entity: Query<(Entity, &TempDojoEntityWrapper)>,
    mut query_player: Query<&mut GameModel>,
    mut commands: Commands,
) {
    for (id, wrapper) in query_temp_dojo_entity.iter() {
        let has_model = wrapper.dojo_entity.models.len() > 0;
        if has_model {
            match wrapper.dojo_entity.models[0].name.as_str() {
                "revolt-Game" => {
                    let new_game: GameModel = wrapper.dojo_entity.clone().into();
                    let mut is_new_game = true;

                    for mut game in query_player.iter_mut() {
                        if game.map_id == new_game.map_id {
                            // sync the game model in bevy with the newer incomeing game model
                            game.id = new_game.id;
                            game.player_1_address = new_game.player_1_address;
                            game.player_2_address = new_game.player_2_address;
                            game.player_3_address = new_game.player_3_address;
                            game.player_4_address = new_game.player_4_address;
                            game.state = new_game.state;
                            game.winner = new_game.winner;
                            is_new_game = false;
                        }
                    }

                    if is_new_game {
                        commands.spawn(new_game);
                    }
                    info!("Despawning game entity: {:?}", id);
                    commands.entity(id).despawn();
                }
                _ => {}
            }
        }
    }
}

impl Into<GameModel> for DojoEntity {
    fn into(self) -> GameModel {
        let dojo_entity = self;

        let id: u32 = member_to_u32(&dojo_entity.models[0].children[0]);
        let player_1_address: Felt =
            member_to_contract_address_to_felt(&dojo_entity.models[0].children[1]);
        let player_2_address: Felt =
            member_to_contract_address_to_felt(&dojo_entity.models[0].children[2]);
        let player_3_address: Felt =
            member_to_contract_address_to_felt(&dojo_entity.models[0].children[3]);
        let player_4_address: Felt =
            member_to_contract_address_to_felt(&dojo_entity.models[0].children[4]);
        let map_id: u32 = member_to_u32(&dojo_entity.models[0].children[5]);
        let state: bool = member_to_bool(&dojo_entity.models[0].children[6]);
        let winner: Felt = member_to_contract_address_to_felt(&dojo_entity.models[0].children[7]);

        GameModel {
            id,
            player_1_address,
            player_2_address,
            player_3_address,
            player_4_address,
            map_id,
            state,
            winner,
        }
    }
}
