use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Player {
    pub game_id: u32,
    pub player_address: u32,
    pub score: u32,
    pub is_alive: bool,
    pub position_x: u32,
    pub position_y: u32,
    pub freeze_moves: u32,
    pub health: u32,
}

impl Player {
    pub fn new(position_x: u32, position_y: u32) -> Self {
        Player {
            game_id: 0,
            player_address: 0,
            score: 0,
            is_alive: true,
            position_x,
            position_y,
            freeze_moves: 0,
            health: 100,
        }
    }
}
