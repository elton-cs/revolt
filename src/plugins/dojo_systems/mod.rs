use account::StarknetRsPlugin;
use bevy::prelude::*;
use create_game::CreateGame;
pub mod account;
pub mod create_game;

pub struct SendTransactionsPlugin;
impl Plugin for SendTransactionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StarknetRsPlugin);
        app.add_plugins(CreateGame);
    }
}
