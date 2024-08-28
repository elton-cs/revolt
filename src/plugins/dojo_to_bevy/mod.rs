use crate::torii::client::TempDojoEntityWrapper;
use bevy::prelude::*;
use map::MapModel;
use player::PlayerModel;
use tile::TileModel;
pub mod map;
pub mod player;
pub mod tile;
pub mod type_extractors;

pub struct RevoltModelsPlugin;
impl Plugin for RevoltModelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, convert_to_bevy);
    }
}

// fn number_of_converted(query: Query<&RevoltTile>) {
//     let value = query.iter().count();
//     info!("Number of converted tiles: {}", value);
// }

fn convert_to_bevy(mut query: Query<(Entity, &TempDojoEntityWrapper)>, mut commands: Commands) {
    for (id, dojo_wrapper) in query.iter_mut() {
        let dojo_entity = dojo_wrapper.dojo_entity.clone();
        let has_model = dojo_entity.models.len() > 0;

        if has_model {
            match dojo_entity.models[0].name.as_str() {
                "revolt-Map" => {
                    let map: MapModel = dojo_entity.into();
                    commands.spawn(map);
                    commands.entity(id).despawn();
                }
                "revolt-Tile" => {
                    let tile: TileModel = dojo_entity.into();
                    commands.spawn(tile);
                    commands.entity(id).despawn();
                }
                "revolt-Player" => {
                    let tile: PlayerModel = dojo_entity.into();
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
