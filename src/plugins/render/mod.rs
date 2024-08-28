use bevy::prelude::*;
use map::MapRendererPlugin;
use player::PlayerRendererPlugin;

pub mod map;
pub mod player;

pub struct DungeonRenderPlugin;

impl Plugin for DungeonRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapRendererPlugin);
        app.add_plugins(PlayerRendererPlugin);
    }
}
