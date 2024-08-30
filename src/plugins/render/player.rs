use crate::{
    plugins::dojo_to_bevy::player::PlayerModel,
    states::GameStates,
    utils::constants::{PLAYER_Z_HEIGHT, TILE_SCALE, TILE_SIZE},
};
use bevy::prelude::*;

pub struct PlayerRendererPlugin;
impl Plugin for PlayerRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_player.run_if(in_state(GameStates::InGame)));
        app.add_systems(Update, update_player_render);
    }
}

#[derive(Component)]
pub struct RenderedPlayer;

#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct PlayerID(Entity);

fn render_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query_player: Query<(Entity, &PlayerModel), Without<RenderedPlayer>>,
) {
    let texture1: Handle<Image> =
        asset_server.load("cooked_by_hpmnk/Empoleon_Idle-Anim_ripped_8x4f_32x.png");
    let texture2: Handle<Image> =
        asset_server.load("cooked_by_hpmnk/Pikachu_Idle-Anim_ripped_8x4f_32x.png");

    let image_handles = [texture1, texture2];

    let map_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 8, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    let mut index = 0;
    for (id, player) in query_player.iter() {
        let (x, y) = (
            player.pos_x as f32 * TILE_SIZE,
            player.pos_y as f32 * TILE_SIZE * -1.0,
        );

        let mut transform = Transform::from_translation(Vec3::new(x, y, PLAYER_Z_HEIGHT));
        transform.scale = TILE_SCALE;

        let texture_atlas = TextureAtlas {
            layout: texture_atlas_layout_handle.clone(),
            index: 0,
        };

        let sprite_bundle = SpriteBundle {
            transform,
            texture: image_handles[index].clone(),
            ..default()
        };

        index += 1;
        commands.spawn((texture_atlas, sprite_bundle, PlayerSprite, PlayerID(id)));
        commands.entity(id).insert(RenderedPlayer);
    }
}

fn update_player_render(
    query_rendered_player: Query<(Entity, &PlayerModel), With<RenderedPlayer>>,
    mut query_transform: Query<(&mut Transform, &PlayerID), With<PlayerSprite>>,
) {
    for (mut transform, parent) in query_transform.iter_mut() {
        let parent_entity_id = parent.0;

        for (id, player) in query_rendered_player.iter() {
            if parent_entity_id == id {
                let (x, y) = (
                    player.pos_x as f32 * TILE_SIZE,
                    player.pos_y as f32 * TILE_SIZE * -1.0,
                );

                transform.translation = Vec3::new(x, y, PLAYER_Z_HEIGHT);
            }
        }
    }
}
