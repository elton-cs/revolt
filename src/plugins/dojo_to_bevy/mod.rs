use crate::torii::client::TempDojoEntityWrapper;
use bevy::prelude::*;
use game::GamePlugin;
use map::MapModel;
use player::PlayerPlugin;
use tile::TileModel;
pub mod game;
pub mod map;
pub mod player;
pub mod tile;
pub mod type_extractors;

pub struct RevoltModelsPlugin;
impl Plugin for RevoltModelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
        app.add_plugins(GamePlugin);
        app.add_systems(Update, convert_to_bevy);
    }
}

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
                _ => {}
            }
        } else {
            info!("Despawning entity with no models");
            commands.entity(id).despawn();
        }
    }
}
