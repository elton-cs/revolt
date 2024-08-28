use crate::{
    plugins::dojo_to_bevy::{map::MapModel, tile::TileModel},
    utils::constants::{
        GROUND_TEXTURE_INDEX, GROUND_Z_HEIGHT, TILE_SCALE, TILE_SIZE, WALL_TEXTURE_INDEX,
        WALL_Z_HEIGHT,
    },
};
use bevy::prelude::*;

pub struct MapRendererPlugin;
impl Plugin for MapRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_walls);
        app.add_systems(Update, render_ground);
    }
}

#[derive(Component)]
pub struct RenderedTile;

#[derive(Component)]
pub struct RenderedGround;

fn render_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query_map: Query<(Entity, &MapModel), Without<RenderedGround>>,
) {
    let map_texture = asset_server.load("2_tiles.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 2, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    let map = query_map.get_single();
    if let Ok((id, map)) = map {
        let max_x = map.cols;
        let max_y = map.rows;

        for x in 0..max_x {
            for y in 0..max_y {
                let (x, y) = (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE * -1.0);

                let mut transform = Transform::from_translation(Vec3::new(x, y, GROUND_Z_HEIGHT));
                transform.scale = TILE_SCALE;

                let texture_atlas = TextureAtlas {
                    layout: texture_atlas_layout_handle.clone(),
                    index: GROUND_TEXTURE_INDEX,
                };

                let sprite_bundle = SpriteBundle {
                    transform,
                    texture: map_texture.clone(),
                    ..default()
                };

                commands.spawn((texture_atlas, sprite_bundle));
            }
        }
        commands.entity(id).insert(RenderedGround);
    }
}

fn render_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut tiles_query: Query<(Entity, &TileModel), Without<RenderedTile>>,
) {
    let map_texture = asset_server.load("2_tiles.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 2, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    for (id, tile) in tiles_query.iter_mut() {
        let (x, y) = (tile.pos_x, tile.pos_y);
        let (x, y) = (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE * -1.0);

        let mut transform = Transform::from_translation(Vec3::new(x, y, WALL_Z_HEIGHT));
        transform.scale = TILE_SCALE;

        let texture_atlas = TextureAtlas {
            layout: texture_atlas_layout_handle.clone(),
            index: WALL_TEXTURE_INDEX,
        };

        let sprite_bundle = SpriteBundle {
            transform,
            texture: map_texture.clone(),
            ..default()
        };

        commands.spawn((texture_atlas, sprite_bundle));
        commands.entity(id).insert(RenderedTile);
    }
}
