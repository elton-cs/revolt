use bevy::prelude::*;

const TILE_SIZE: f32 = 8.0;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_map, render_map).chain());
    }
}

#[derive(Resource)]
pub struct MapResource {
    matrix: [[u32; 9]; 9],
}

pub fn generate_map(mut commands: Commands) {
    let map_matrix = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 0, 0, 0, 1, 1, 1],
        [1, 1, 0, 0, 0, 0, 0, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 1, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 0, 0, 0, 0, 0, 1, 1],
        [1, 1, 1, 0, 0, 0, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    commands.insert_resource(MapResource { matrix: map_matrix });
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
