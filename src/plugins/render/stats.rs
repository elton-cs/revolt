use crate::{
    plugins::dojo_to_bevy::player::PlayerModel,
    utils::constants::{PLAYER_Z_HEIGHT, TILE_SCALE, TILE_SIZE},
};
use bevy::prelude::*;

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_stats);
    }
}

fn print_stats(query: Query<&PlayerModel>) {
    for player in query.iter() {
        println!(
            "Player: {} | Score: {} | State: {} | Pos: ({}, {}) | Freeze: {} | Health: {}",
            player.game_id,
            player.score,
            player.state,
            player.pos_x,
            player.pos_y,
            player.freeze,
            player.health
        );
    }
}

fn display_stats(query: Query<&PlayerModel>) {
    for player in query.iter() {
        println!(
            "Player: {} | Score: {} | State: {} | Pos: ({}, {}) | Freeze: {} | Health: {}",
            player.player_address.to_string().split_off(5),
            player.score,
            player.state,
            player.pos_x,
            player.pos_y,
            player.freeze,
            player.health
        );
    }
}
