use bevy::prelude::*;

use crate::{
    states::GameStates,
    utils::constants::{MAX_HEALTH, PLAYER_STATS_Z_HEIGHT},
};

use super::dojo_to_bevy::player::PlayerModel;

pub struct StatsPlugin;
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            init_stat_display.run_if(in_state(GameStates::InGame)),
        );
        // app.add_systems(Startup, setup);
    }
}

#[derive(Component, Debug)]
pub struct PlayerStats;

#[derive(Component, Debug)]
pub struct ParentStatsId(Entity);

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let text = "Player Stats";
//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.0),
//                 height: Val::Percent(100.0),
//                 justify_content: JustifyContent::Default,
//                 align_items: AlignItems::Center,
//                 flex_direction: FlexDirection::Column,
//                 ..default()
//             },
//             transform: Transform::from_translation(Vec3::new(10.0, 0.0, PLAYER_STATS_Z_HEIGHT)),
//             ..default()
//         })
//         .with_children(|p| {
//             for text in ["Player Stats", "Health", "Mana", "Attack", "Defense"] {
//                 p.spawn(TextBundle::from_section(
//                     text,
//                     TextStyle {
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         font_size: 24.0,
//                         color: Color::srgb(0.9, 0.9, 0.9),
//                     },
//                 ));
//             }
//         });
// }

fn init_stat_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_stats: Query<(Entity, &PlayerModel), Without<PlayerStats>>,
    // query_existing_stats: Query<Entity, With<PlayerStats>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_size = 5.0;
    // let text_style = TextStyle {
    //     font: font.clone(),
    //     font_size: 10.0,
    //     ..default()
    // };
    let text_justification = JustifyText::Center;

    let mut pos_index = 0;
    for (id, player) in query_stats.iter() {
        let health = player.health;
        let health_text = format!("Health: {}/{}", health, MAX_HEALTH);
        let health_section = TextSection::new(
            health_text.clone(),
            TextStyle {
                font: font.clone(),
                font_size,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        );
        let (x, y) = (player.pos_x, player.pos_y);
        let position_text = format!("Position: ({}, {})", x, y);
        let position_section = TextSection::new(
            position_text.clone(),
            TextStyle {
                font: font.clone(),
                font_size,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        );

        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([position_section, health_section])
                    // text: Text::from_section(health_text, text_style.clone())
                    .with_justify(JustifyText::Center),
                transform: Transform::from_translation(Vec3::new(
                    (pos_index) as f32 * 250.0,
                    20.0,
                    PLAYER_STATS_Z_HEIGHT,
                )),
                ..default()
            },
            ParentStatsId(id),
        ));
        pos_index += 1;
        commands.entity(id).insert(PlayerStats);
    }
}

fn update_stats() {}
