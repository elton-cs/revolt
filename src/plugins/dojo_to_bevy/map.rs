use crate::torii::client::TempDojoEntityWrapper;
use bevy::prelude::*;
use torii_grpc::types::schema::Entity as DojoEntity;

pub struct BevyMapPlugin;
impl Plugin for BevyMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, convert_to_bevy);
        app.add_systems(Update, number_of_converted);
    }
}

#[derive(Component)]
pub struct Map {
    pub id: u32,
    pub rows: u8,
    pub cols: u8,
}

#[derive(Component)]
pub struct Tile {
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
                    let map: Map = dojo_entity.into();
                    commands.spawn(map);
                    commands.entity(id).despawn();
                }
                "revolt-Tile" => {
                    let tile: Tile = dojo_entity.into();
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

fn number_of_converted(query: Query<&Tile>) {
    let value = query.iter().count();
    info!("Number of converted tiles: {}", value);
}

impl Into<Map> for DojoEntity {
    fn into(self) -> Map {
        let dojo_entity = self;

        let id = dojo_entity.models[0].children[0]
            .ty
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap()
            .clone();
        let rows = dojo_entity.models[0].children[1]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap()
            .clone();
        let cols = dojo_entity.models[0].children[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap()
            .clone();

        Map { id, rows, cols }
    }
}

impl Into<Tile> for DojoEntity {
    fn into(self) -> Tile {
        let dojo_entity = self;

        let map_id = dojo_entity.models[0].children[0]
            .ty
            .as_primitive()
            .unwrap()
            .as_u32()
            .unwrap()
            .clone();
        let pos_x = dojo_entity.models[0].children[1]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap()
            .clone();
        let pos_y = dojo_entity.models[0].children[2]
            .ty
            .as_primitive()
            .unwrap()
            .as_u8()
            .unwrap()
            .clone();
        let value = dojo_entity.models[0].children[3]
            .ty
            .as_enum()
            .unwrap()
            .option
            .unwrap()
            .clone();

        Tile {
            map_id,
            pos_x,
            pos_y,
            value,
        }
    }
}
