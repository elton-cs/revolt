use crate::{
    plugins::dojo_to_bevy::player::PlayerModel,
    utils::constants::{PLAYER_Z_HEIGHT, TILE_SCALE, TILE_SIZE},
};
use bevy::prelude::*;

pub struct PlayerRendererPlugin;
impl Plugin for PlayerRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_player);
    }
}

#[derive(Component)]
pub struct RenderedPlayer;

fn render_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query_player: Query<(Entity, &PlayerModel), Without<RenderedPlayer>>,
) {
    let map_texture = asset_server.load("cooked_by_hpmnk/Empoleon_Idle-Anim_ripped_8x4f_32x.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 8, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    let player = query_player.get_single();
    if let Ok((id, player)) = player {
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
            texture: map_texture.clone(),
            ..default()
        };

        commands.spawn((texture_atlas, sprite_bundle));
        commands.entity(id).insert(RenderedPlayer);
    }
}
