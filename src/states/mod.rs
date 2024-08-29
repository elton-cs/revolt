use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameStates {
    MainMenu,
    InGame,
}

pub struct GameStatesPlugin;
impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameStates::MainMenu);
    }
}
