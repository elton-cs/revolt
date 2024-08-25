use super::constants::{PLAYER_CAMERA_SCALE, PLAYER_SCALE, TILE_SIZE};
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_player, render_player).chain());
        app.add_systems(PostStartup, player_camera);
    }
}

#[derive(Debug, Component)]
pub struct Player {
    game_id: u32,
    player_address: u32,
    score: u32,
    is_alive: bool,
    position_x: u32,
    position_y: u32,
    freeze_moves: u32,
    health: u32,
}

#[derive(Debug, Component)]
struct PlayerTag;

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

fn generate_player(mut commands: Commands) {
    let player = Player::new(3, 42);
    commands.spawn(player);
}

fn render_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<&Player>,
) {
    let player_texture = asset_server.load("charizard_walk_down.png");
    let player_layout = TextureAtlasLayout::from_grid(UVec2::new(40, 40), 4, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(player_layout);

    for player in query.iter() {
        let mut transform = Transform::from_translation(Vec3::new(
            player.position_x as f32 * TILE_SIZE,
            player.position_y as f32 * TILE_SIZE,
            2.0,
        ));
        transform.scale = PLAYER_SCALE;

        let texture_atlas = TextureAtlas {
            layout: texture_atlas_layout_handle.clone(),
            index: 0,
        };

        let sprite_bundle = SpriteBundle {
            transform,
            texture: player_texture.clone(),
            ..default()
        };

        commands.spawn((texture_atlas, sprite_bundle, PlayerTag));
    }
}

fn player_camera(mut commands: Commands, query_player: Query<&Transform, With<PlayerTag>>) {
    let mut camera_bundle = Camera2dBundle::default();
    let player = query_player.single();

    camera_bundle.transform.translation = player.translation;
    camera_bundle.projection.scale = PLAYER_CAMERA_SCALE;
    commands.spawn(camera_bundle);
}
