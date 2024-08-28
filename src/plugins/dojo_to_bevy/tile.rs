use super::type_extractors::{member_to_enum_to_u8, member_to_u32, member_to_u8};
use bevy::prelude::*;
use torii_grpc::types::schema::Entity as DojoEntity;

#[derive(Component)]
pub struct TileModel {
    pub map_id: u32,
    pub pos_x: u8,
    pub pos_y: u8,
    pub value: u8,
}

impl Into<TileModel> for DojoEntity {
    fn into(self) -> TileModel {
        let dojo_entity = self;

        let map_id = member_to_u32(&dojo_entity.models[0].children[0]);
        let pos_x = member_to_u8(&dojo_entity.models[0].children[1]);
        let pos_y = member_to_u8(&dojo_entity.models[0].children[2]);
        let value = member_to_enum_to_u8(&dojo_entity.models[0].children[3]);

        TileModel {
            map_id,
            pos_x,
            pos_y,
            value,
        }
    }
}
