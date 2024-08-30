use crate::plugins::dojo_to_bevy::player::PlayerModel;
use bevy::prelude::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_model);
        app.add_systems(Update, display_num_entities);
    }
}

fn player_model(query: Query<&PlayerModel, Changed<PlayerModel>>) {
    for player in query.iter() {
        info!("Player Model: {:?}", player);
    }
}

fn display_num_entities(query: Query<Entity>) {
    let num_entities = query.iter().count();
    info!("Total entities: {}", num_entities);
}
