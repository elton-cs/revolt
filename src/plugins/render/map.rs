use crate::{
    plugins::dojo_to_bevy::map::Tile,
    utils::constants::{MAP_SIZE, TILE_SCALE, TILE_SIZE},
};
use bevy::prelude::*;

pub struct MapRendererPlugin;
impl Plugin for MapRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_tiles);
    }
}

#[derive(Component)]
pub struct RenderedTile;

fn render_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut tiles_query: Query<(Entity, &Tile), Without<RenderedTile>>,
) {
    let map_texture = asset_server.load("2_tiles.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 2, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    for (id, tile) in tiles_query.iter_mut() {
        let (x, y) = (tile.pos_x, tile.pos_y);
        let flip_y = MAP_SIZE.1 - y - 1;
        let (x, y) = (x as f32 * TILE_SIZE, flip_y as f32 * TILE_SIZE);

        let mut transform = Transform::from_translation(Vec3::new(x, y, 0.0));
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
