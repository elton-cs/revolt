use bevy::prelude::*;
use map::MapRendererPlugin;

pub mod map;

pub struct DungeonRenderPlugin;

impl Plugin for DungeonRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MapRendererPlugin);
    }
}
