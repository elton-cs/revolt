use crate::torii::client::TempDojoEntityWrapper;
use bevy::prelude::*;
use torii_grpc::types::schema::Entity as DojoEntity;

use super::type_extractors::{member_to_enum_to_u8, member_to_u32, member_to_u8};

pub struct DojoMapModels;
impl Plugin for DojoMapModels {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, convert_to_bevy);
        // app.add_systems(Update, number_of_converted);
    }
}

#[derive(Component)]
pub struct RevoltMap {
    pub id: u32,
    pub rows: u8,
    pub cols: u8,
}

#[derive(Component)]
pub struct RevoltTile {
    pub map_id: u32,
    pub pos_x: u8,
    pub pos_y: u8,
    // pub value: TileValue,
    pub value: u8,
}

// pub enum TileValue {
//     None,
//     Wall,
//     Path,
// }

fn convert_to_bevy(mut query: Query<(Entity, &TempDojoEntityWrapper)>, mut commands: Commands) {
    for (id, dojo_wrapper) in query.iter_mut() {
        let dojo_entity = dojo_wrapper.dojo_entity.clone();
        let has_model = dojo_entity.models.len() > 0;

        if has_model {
            match dojo_entity.models[0].name.as_str() {
                "revolt-Map" => {
                    let map: RevoltMap = dojo_entity.into();
                    commands.spawn(map);
                    commands.entity(id).despawn();
                }
                "revolt-Tile" => {
                    let tile: RevoltTile = dojo_entity.into();
                    commands.spawn(tile);
                    commands.entity(id).despawn();
                }
                _ => {
                    error!(
                        "Conversion pending implementation. Unknown entity: {}",
                        dojo_entity.models[0].name
                    );
                }
            }
        } else {
            info!("Despawning entity with no models");
            commands.entity(id).despawn();
        }
    }
}

fn number_of_converted(query: Query<&RevoltTile>) {
    let value = query.iter().count();
    info!("Number of converted tiles: {}", value);
}

impl Into<RevoltMap> for DojoEntity {
    fn into(self) -> RevoltMap {
        let dojo_entity = self;

        let id = member_to_u32(&dojo_entity.models[0].children[0]);
        let rows = member_to_u8(&dojo_entity.models[0].children[1]);
        let cols = member_to_u8(&dojo_entity.models[0].children[2]);

        RevoltMap { id, rows, cols }
    }
}

impl Into<RevoltTile> for DojoEntity {
    fn into(self) -> RevoltTile {
        let dojo_entity = self;

        let map_id = member_to_u32(&dojo_entity.models[0].children[0]);
        let pos_x = member_to_u8(&dojo_entity.models[0].children[1]);
        let pos_y = member_to_u8(&dojo_entity.models[0].children[2]);
        let value = member_to_enum_to_u8(&dojo_entity.models[0].children[3]);

        RevoltTile {
            map_id,
            pos_x,
            pos_y,
            value,
        }
    }
}
