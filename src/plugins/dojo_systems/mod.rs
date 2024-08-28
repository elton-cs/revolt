use account::StarknetRsPlugin;
use attack_enemy::AttackEnemy;
use bevy::prelude::*;
use create_game::CreateGame;
use move_player::MovePlayer;
pub mod account;
pub mod attack_enemy;
pub mod create_game;
pub mod join_game;
pub mod move_player;

pub struct SendTransactionsPlugin;
impl Plugin for SendTransactionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StarknetRsPlugin);
        app.add_plugins(CreateGame);
        app.add_plugins(MovePlayer);
        app.add_plugins(AttackEnemy);
    }
}
