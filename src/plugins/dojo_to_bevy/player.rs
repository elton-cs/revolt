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

#[derive(Component)]
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

// NOTE: Should only spawn a player once
fn spawn_or_update_player(
    query_temp_dojo_entity: Query<&TempDojoEntityWrapper>,
    mut query_player: Query<&mut PlayerModel>,
    mut commands: Commands,
) {
    let player_count = query_player.iter().count();

    for wrapper in query_temp_dojo_entity.iter() {
        let has_model = wrapper.dojo_entity.models.len() > 0;
        if has_model {
            // let dojo_entity = wrapper.dojo_entity.clone();
            match wrapper.dojo_entity.models[0].name.as_str() {
                "revolt-Player" => {
                    let new_player: PlayerModel = wrapper.dojo_entity.clone().into();
                    if player_count > 0 {
                        let mut player = query_player.single_mut();
                        // Sync with player dojo entity
                        player.game_id = new_player.game_id;
                        player.player_address = new_player.player_address;
                        player.pos_x = new_player.pos_x;
                        player.pos_y = new_player.pos_y;
                        player.score = new_player.score;
                        player.state = new_player.state;
                        player.freeze = new_player.freeze;
                        player.health = new_player.health;
                    } else {
                        commands.spawn(new_player);
                    }
                }
                _ => {}
            }
        }
    }
}

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
