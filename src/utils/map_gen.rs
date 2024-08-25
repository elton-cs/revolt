use bevy::prelude::*;
use rand::Rng;

use super::img_to_matrix::{convertor, pixel_convertor};

const TILE_SIZE: f32 = 8.0;
const WIDTH: usize = 20;
const HEIGHT: usize = 20;
const WALL_PROBABILITY: f32 = 0.45;
const SMOOTHING_ITERATIONS: usize = 5;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, (generate_manual_map, render_map).chain());
        // app.add_systems(Startup, (generate_random_map, render_map).chain());
        app.add_systems(Startup, (gen_map_1, render_map).chain());
    }
}

#[derive(Resource)]
pub struct MapResource {
    matrix: Vec<Vec<u32>>,
}

pub fn generate_manual_map(mut commands: Commands) {
    let map_matrix = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 0, 0, 0, 1, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 0, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 1, 1, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 0, 0, 0, 1, 1],
        vec![1, 1, 1, 0, 0, 0, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    commands.insert_resource(MapResource { matrix: map_matrix });
}

pub fn gen_map_1(mut commands: Commands) {
    // let map_matrix = convertor("src/dungeon_35x45.png");
    let map_matrix = pixel_convertor("src/dungeon_pixelated.png");

    commands.insert_resource(MapResource { matrix: map_matrix });
}

pub fn generate_random_map(mut commands: Commands) {
    let map = generate_dungeon_map();

    commands.insert_resource(MapResource { matrix: map });
}

fn render_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    map: Res<MapResource>,
) {
    // let map_texture = asset_server.load("cave_32c_24r.png");
    // let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 32, 24, None, None);

    let map_texture = asset_server.load("2_tiles.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 2, 1, None, None);

    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    for (y, row) in map.matrix.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            // let tile_index = match tile {
            //     0 => 134,
            //     1 => 353,
            //     // 0 => 0,
            //     // 1 => 1,
            //     _ => 0,
            // };

            let mut transform = Transform::from_translation(Vec3::new(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
                0.0,
            ));
            transform.scale = Vec3::splat(0.5);

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

            match tile {
                1 => {
                    let mut transform = Transform::from_translation(Vec3::new(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        1.0,
                    ));
                    transform.scale = Vec3::splat(0.5);
                    let texture_atlas = TextureAtlas {
                        layout: texture_atlas_layout_handle.clone(),
                        index: 1,
                    };
                    let sprite_bundle = SpriteBundle {
                        transform,
                        texture: map_texture.clone(),
                        ..default()
                    };

                    commands.spawn((texture_atlas, sprite_bundle));
                }
                _ => {}
            };
        }
    }
}

fn generate_dungeon_map() -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut map = vec![vec![0; WIDTH]; HEIGHT];

    // Initialize the map with random walls
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if rng.gen::<f32>() < WALL_PROBABILITY {
                map[y][x] = 1;
            }
        }
    }

    // Apply cellular automaton rules
    for _ in 0..SMOOTHING_ITERATIONS {
        map = smooth_map(&map);
    }

    map
}

fn smooth_map(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new_map = map.clone();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let wall_count = count_walls_around(map, x, y);

            if wall_count > 4 {
                new_map[y][x] = 1;
            } else if wall_count < 4 {
                new_map[y][x] = 0;
            }
        }
    }

    new_map
}

fn count_walls_around(map: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                if map[ny as usize][nx as usize] == 1 {
                    count += 1;
                }
            } else {
                count += 1; // Treat out-of-bounds as walls
            }
        }
    }

    count
}
