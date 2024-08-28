use super::type_extractors::{
    member_to_bool, member_to_contract_address_to_felt, member_to_u16, member_to_u32, member_to_u8,
};
use bevy::prelude::*;
use starknet_crypto::Felt;
use torii_grpc::types::schema::Entity as DojoEntity;

#[derive(Component)]
pub struct PlayerModel {
    pub game_id: u32,
    // player_address is a contract address expressed as a Felt type
    pub player_address: Felt,
    pub score: u16,
    pub state: bool,
    pub pos_x: u8,
    pub pos_y: u8,
    pub freeze: u8,
    pub health: u8,
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
