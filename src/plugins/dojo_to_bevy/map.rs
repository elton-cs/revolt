use super::type_extractors::{member_to_u32, member_to_u8};
use bevy::prelude::*;
use torii_grpc::types::schema::Entity as DojoEntity;

#[derive(Component)]
pub struct MapModel {
    pub id: u32,
    pub rows: u8,
    pub cols: u8,
}

impl Into<MapModel> for DojoEntity {
    fn into(self) -> MapModel {
        let dojo_entity = self;

        let id = member_to_u32(&dojo_entity.models[0].children[0]);
        let rows = member_to_u8(&dojo_entity.models[0].children[1]);
        let cols = member_to_u8(&dojo_entity.models[0].children[2]);

        MapModel { id, rows, cols }
    }
}
