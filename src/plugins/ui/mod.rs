use bevy::prelude::*;
use create_game::CreateGamePlugin;
use join_game::JoinGamePlugin;
pub mod create_game;
pub mod join_game;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CreateGamePlugin);
        app.add_plugins(JoinGamePlugin);
    }
}
