use crate::utils::constants::{BUTTONS_SCALE, CREATE_BUTTON_LOCATION};
use bevy::prelude::*;

pub struct CreateGamePlugin;
impl Plugin for CreateGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_button);
    }
}

#[derive(Component)]
pub struct CreateGameButton;

fn render_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut transform = Transform::from_translation(CREATE_BUTTON_LOCATION);
    transform.scale = BUTTONS_SCALE;

    let sprite = SpriteBundle {
        texture: asset_server.load("create_game_button.png"),
        transform,
        ..default()
    };

    commands.spawn((sprite, CreateGameButton));
}
