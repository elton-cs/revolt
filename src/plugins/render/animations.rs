use crate::{
    plugins::dojo_to_bevy::player::PlayerModel,
    utils::constants::{ATTACK_Z_HEIGHT, PLAYER_Z_HEIGHT, TILE_SCALE, TILE_SIZE},
};
use bevy::prelude::*;

use super::player::RenderedPlayer;

pub struct AnimationsPlugin;
impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggerAttackAnimation>();
        app.add_systems(Update, check_if_player_frozen);
        app.add_systems(Update, (clear_attacks, animate_attack).chain());
    }
}

#[derive(Event)]
pub struct TriggerAttackAnimation {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct AttackVisible;

fn check_if_player_frozen(
    mut query_player: Query<(Entity, &mut PlayerModel), With<RenderedPlayer>>,
    mut event_writer: EventWriter<TriggerAttackAnimation>,
) {
    for (id, mut player) in query_player.iter_mut() {
        if player.freeze == 5 {
            let x = player.pos_x;
            let y = player.pos_y;
            event_writer.send(TriggerAttackAnimation { x, y });
            player.freeze = 4;
        }
    }
}

fn clear_attacks(
    query_attacks: Query<Entity, With<AttackVisible>>,
    query_players: Query<&mut PlayerModel>,
    mut commands: Commands,
) {
    for player in query_players.iter() {
        if player.freeze < 4 {
            for entity in query_attacks.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn animate_attack(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut event_reader: EventReader<TriggerAttackAnimation>,
) {
    let texture2: Handle<Image> =
        asset_server.load("cooked_by_hpmnk/GroundPoundV2_Sheet_1X13f_48x.png");
    let map_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 13, 1, None, None);
    let texture_atlas_layout_handle = texture_atlas_layouts.add(map_layout);

    for data in event_reader.read() {
        let (x, y) = (data.x as f32 * TILE_SIZE, data.y as f32 * TILE_SIZE * -1.0);

        let mut transform = Transform::from_translation(Vec3::new(x, y, ATTACK_Z_HEIGHT));
        transform.scale = TILE_SCALE;

        let texture_atlas = TextureAtlas {
            layout: texture_atlas_layout_handle.clone(),
            index: 3,
        };

        let sprite_bundle = SpriteBundle {
            transform,
            texture: texture2.clone(),
            ..default()
        };

        commands.spawn((texture_atlas, sprite_bundle, AttackVisible));
    }
}

// #[derive(Component, Clone)]
// struct AnimationConfig {
//     first_sprite_index: usize,
//     last_sprite_index: usize,
//     fps: u8,
//     frame_timer: Timer,
// }

// // This system runs when the user clicks the left arrow key or right arrow key
// fn trigger_animation<S: Component>(mut query: Query<&mut AnimationConfig, With<S>>) {
//     // we expect the Component of type S to be used as a marker Component by only a single entity
//     let mut animation = query.single_mut();
//     // we create a new timer when the animation is triggered
//     animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
// }

// impl AnimationConfig {
//     fn new(first: usize, last: usize, fps: u8) -> Self {
//         Self {
//             first_sprite_index: first,
//             last_sprite_index: last,
//             fps,
//             frame_timer: Self::timer_from_fps(fps),
//         }
//     }

//     fn timer_from_fps(fps: u8) -> Timer {
//         Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
//     }
// }

// // This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// // `last_sprite_index` (both defined in `AnimationConfig`).
// fn execute_animations(
//     time: Res<Time>,
//     mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
// ) {
//     for (mut config, mut atlas) in &mut query {
//         // we track how long the current sprite has been displayed for
//         config.frame_timer.tick(time.delta());

//         // If it has been displayed for the user-defined amount of time (fps)...
//         if config.frame_timer.just_finished() {
//             if atlas.index == config.last_sprite_index {
//                 // ...and it IS the last frame, then we move back to the first frame and stop.
//                 atlas.index = config.first_sprite_index;
//             } else {
//                 // ...and it is NOT the last frame, then we move to the next frame...
//                 atlas.index += 1;
//                 // ...and reset the frame timer to start counting all over again
//                 config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
//             }
//         }
//     }
// }

// #[derive(Component)]
// struct LeftSprite;

// #[derive(Component)]
// struct RightSprite;

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
//     query_player: Query<(Entity, &PlayerModel), With<RenderedPlayer>>,
// ) {
//     // load the sprite sheet using the `AssetServer`
//     // let texture = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
//     let texture = asset_server.load("cooked_by_hpmnk/GroundPoundV2_Sheet_1X13f_48x.png");

//     // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
//     let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 13, 1, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);

//     // the first (left-hand) sprite runs at 10 FPS
//     let animation_config_1 = AnimationConfig::new(0, 12, 30);

//     let (id, player) = query_player.iter().next().unwrap();

//     let (x, y) = (
//         player.pos_x as f32 * TILE_SCALE.x,
//         player.pos_y as f32 * TILE_SCALE.y * -1.0,
//     );

//     // create the first (left-hand) sprite
//     commands.spawn((
//         SpriteBundle {
//             // transform: Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(
//             transform: Transform::from_scale(TILE_SCALE).with_translation(Vec3::new(
//                 x,
//                 y,
//                 PLAYER_STATS_Z_HEIGHT,
//             )),
//             texture: texture.clone(),
//             ..default()
//         },
//         TextureAtlas {
//             layout: texture_atlas_layout.clone(),
//             index: animation_config_1.first_sprite_index,
//         },
//         LeftSprite,
//         animation_config_1,
//     ));

//     // the second (right-hand) sprite runs at 20 FPS
//     let animation_config_2 = AnimationConfig::new(4, 7, 10);

//     // create the second (right-hand) sprite
//     // commands.spawn((
//     //     SpriteBundle {
//     //         transform: Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(
//     //             50.0,
//     //             0.0,
//     //             PLAYER_STATS_Z_HEIGHT,
//     //         )),
//     //         texture: texture.clone(),
//     //         ..default()
//     //     },
//     //     TextureAtlas {
//     //         layout: texture_atlas_layout.clone(),
//     //         index: animation_config_2.first_sprite_index,
//     //     },
//     //     RightSprite,
//     //     animation_config_2,
//     // ));

//     // create a minimal UI explaining how to interact with the example
//     commands.spawn(TextBundle {
//         text: Text::from_section(
//             "Left Arrow Key: Animate Left Sprite\nRight Arrow Key: Animate Right Sprite",
//             TextStyle::default(),
//         ),
//         style: Style {
//             position_type: PositionType::Absolute,
//             top: Val::Px(12.0),
//             left: Val::Px(12.0),
//             ..default()
//         },
//         ..default()
//     });
// }
