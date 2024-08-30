use crate::{
    plugins::dojo_to_bevy::{map::MapModel, tile::TileModel},
    utils::constants::{GROUND_Z_HEIGHT, TILE_SCALE, TILE_SIZE, WALL_TEXTURE_INDEX, WALL_Z_HEIGHT},
};
use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

pub struct MapRendererPlugin;
impl Plugin for MapRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_walls);
        // app.add_systems(Update, render_ground);
        app.add_systems(Startup, render_ground_manual);
    }
}

#[derive(Component)]
pub struct RenderedTile;

#[derive(Component)]
pub struct RenderedGround;

fn render_ground_manual(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // let map_texture = asset_server.load("2_tiles.png");
    // let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 2, 1, None, None);

    let map_texture = asset_server.load("cooked_by_hpmnk/PokeDojo_Tileset.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 5, 3, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    // hardcoding the map size like a chad developer
    let max_x = 35;
    let max_y = 19;

    let mut flip = false;
    let light_tile_indices = [0, 1, 2, 3, 4];
    let dark_tile_indices = [5, 6, 7, 8, 9];
    // let light_tile = 0;
    // let dark_tile = 5;

    let mut rng = thread_rng();

    for x in 0..max_x {
        for y in 0..max_y {
            let (x, y) = (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE * -1.0);

            let mut transform = Transform::from_translation(Vec3::new(x, y, GROUND_Z_HEIGHT));
            transform.scale = TILE_SCALE;

            let tile_set = match flip {
                true => light_tile_indices,
                false => dark_tile_indices,
            };
            let random_tile = tile_set.choose(&mut rng).unwrap();

            let texture_atlas = TextureAtlas {
                layout: texture_atlas_layout_handle.clone(),
                // index: GROUND_TEXTURE_INDEX,
                index: *random_tile,
            };
            flip = !flip;

            let sprite_bundle = SpriteBundle {
                transform,
                texture: map_texture.clone(),
                ..default()
            };

            commands.spawn((texture_atlas, sprite_bundle));
        }
    }
}

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
                    index: WALL_TEXTURE_INDEX,
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
    // let map_texture = asset_server.load("2_tiles.png");
    // let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 2, 1, None, None);

    let map_texture = asset_server.load("cooked_by_hpmnk/PokeDojo_Tileset.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 5, 3, None, None);

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
