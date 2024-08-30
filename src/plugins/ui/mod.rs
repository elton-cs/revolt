use bevy::prelude::*;
use game_menu::GameMenuPlugin;
pub mod game_menu;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameMenuPlugin);
    }
}
