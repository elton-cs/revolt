use super::type_extractors::{
    member_to_bool, member_to_contract_address_to_felt, member_to_u16, member_to_u32, member_to_u8,
};
use crate::torii::client::TempDojoEntityWrapper;
use bevy::prelude::*;
use starknet_crypto::Felt;
use torii_grpc::types::schema::Entity as DojoEntity;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_or_update_player);
    }
}

#[derive(Component, Debug)]
pub struct PlayerModel {
    pub game_id: u32,
    // NOTE: player_address is a contract address but is expressed as a Felt type
    pub player_address: Felt,
    pub score: u16,
    pub state: bool,
    pub pos_x: u8,
    pub pos_y: u8,
    pub freeze: u8,
    pub health: u8,
}

// NOTE: Should only spawn up to 4 players
fn spawn_or_update_player(
    query_temp_dojo_entity: Query<(Entity, &TempDojoEntityWrapper)>,
    mut query_player: Query<&mut PlayerModel>,
    mut commands: Commands,
) {
    for (id, wrapper) in query_temp_dojo_entity.iter() {
        let has_model = wrapper.dojo_entity.models.len() > 0;
        if has_model {
            match wrapper.dojo_entity.models[0].name.as_str() {
                "revolt-Player" => {
                    let new_player: PlayerModel = wrapper.dojo_entity.clone().into();
                    let mut is_new_player = true;

                    for mut player_model_in_bevy in query_player.iter_mut() {
                        if player_model_in_bevy.player_address == new_player.player_address {
                            // sync the player model in bevy with the new player
                            player_model_in_bevy.game_id = new_player.game_id;
                            player_model_in_bevy.player_address = new_player.player_address;
                            player_model_in_bevy.pos_x = new_player.pos_x;
                            player_model_in_bevy.pos_y = new_player.pos_y;
                            player_model_in_bevy.score = new_player.score;
                            player_model_in_bevy.state = new_player.state;
                            player_model_in_bevy.freeze = new_player.freeze;
                            player_model_in_bevy.health = new_player.health;
                            is_new_player = false;
                        }
                    }

                    if is_new_player {
                        commands.spawn(new_player);
                    }
                    info!("Despawning player entity: {:?}", id);
                    commands.entity(id).despawn();
                }
                _ => {}
            }
        }
    }
}

// fn clean_up_duplicate_entities(
//     query_temp_dojo_entity: Query<(Entity, &TempDojoEntityWrapper)>,
//     mut query_player: Query<&mut PlayerModel>,
// ) {
// }

impl Into<PlayerModel> for DojoEntity {
    fn into(self) -> PlayerModel {
        let dojo_entity = self;

        let game_id = member_to_u32(&dojo_entity.models[0].children[0]);
        let player_address = member_to_contract_address_to_felt(&dojo_entity.models[0].children[1]);
        let score = member_to_u16(&dojo_entity.models[0].children[2]);
        let state = member_to_bool(&dojo_entity.models[0].children[3]);
        let pos_x = member_to_u8(&dojo_entity.models[0].children[4]);
        let pos_y = member_to_u8(&dojo_entity.models[0].children[5]);
        let freeze = member_to_u8(&dojo_entity.models[0].children[6]);
        let health = member_to_u8(&dojo_entity.models[0].children[7]);

        PlayerModel {
            game_id,
            player_address,
            score,
            state,
            pos_x,
            pos_y,
            freeze,
            health,
        }
    }
}
