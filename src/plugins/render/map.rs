use crate::{
    plugins::dojo_to_bevy::{map::MapModel, tile::TileModel},
    utils::constants::{
        GROUND_TEXTURE_INDEX, GROUND_Z_HEIGHT, MAP_SIZE, TILE_SCALE, TILE_SIZE, WALL_TEXTURE_INDEX,
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

    // // let (map_max_x, map_max_y) = MAP_SIZE;
    // let mut map_max_x: u8 = 0;
    // let mut map_max_y: u8 = 0;

    // for (id, map) in query_map.iter() {
    //     // TODO: figure out why the x and y values need to be flipped for proper rendering
    //     map_max_x = map.rows;
    //     map_max_y = map.cols;
    //     commands.entity(id).insert(RenderedGround);
    // }

    // for x in 0..map_max_x {
    //     for y in 0..map_max_y {
    //         let (x, y) = (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);

    //         let mut transform = Transform::from_translation(Vec3::new(x, y, GROUND_Z_HEIGHT));
    //         transform.scale = TILE_SCALE;

    //         let texture_atlas = TextureAtlas {
    //             layout: texture_atlas_layout_handle.clone(),
    //             index: GROUND_TEXTURE_INDEX,
    //         };

    //         let sprite_bundle = SpriteBundle {
    //             transform,
    //             texture: map_texture.clone(),
    //             ..default()
    //         };

    //         commands.spawn((texture_atlas, sprite_bundle));
    //     }
    // }
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
        // let flip_y = MAP_SIZE.1 - y - 1;
        // let (x, y) = (x as f32 * TILE_SIZE, flip_y as f32 * TILE_SIZE);

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

    // for (y, row) in map.matrix.iter().enumerate() {
    //     for (x, tile) in row.iter().enumerate() {
    //         let mut transform = Transform::from_translation(Vec3::new(
    //             x as f32 * TILE_SIZE,
    //             y as f32 * TILE_SIZE,
    //             0.0,
    //         ));
    //         transform.scale = TILE_SCALE;

    //         let texture_atlas = TextureAtlas {
    //             layout: texture_atlas_layout_handle.clone(),
    //             index: 0,
    //         };

    //         let sprite_bundle = SpriteBundle {
    //             transform,
    //             texture: map_texture.clone(),
    //             ..default()
    //         };

    //         commands.spawn((texture_atlas, sprite_bundle));

    //         match tile {
    //             1 => {
    //                 let mut transform = Transform::from_translation(Vec3::new(
    //                     x as f32 * TILE_SIZE,
    //                     y as f32 * TILE_SIZE,
    //                     1.0,
    //                 ));
    //                 transform.scale = Vec3::splat(0.5);
    //                 let texture_atlas = TextureAtlas {
    //                     layout: texture_atlas_layout_handle.clone(),
    //                     index: 1,
    //                 };
    //                 let sprite_bundle = SpriteBundle {
    //                     transform,
    //                     texture: map_texture.clone(),
    //                     ..default()
    //                 };

    //                 commands.spawn((texture_atlas, sprite_bundle));
    //             }
    //             _ => {}
    //         };
    //     }
    // }
}
