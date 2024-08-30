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
        app.add_systems(Update, update_stats.run_if(in_state(GameStates::InGame)));
    }
}

#[derive(Component, Debug)]
pub struct PlayerStats;

#[derive(Component, Debug)]
pub struct ParentStatsId(pub Entity);

fn init_stat_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_stats: Query<(Entity, &PlayerModel), Without<PlayerStats>>,
    // query_existing_stats: Query<Entity, With<PlayerStats>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_size = 10.0;

    let mut pos_index = 0;
    for (id, player) in query_stats.iter() {
        let health = player.health;
        let health_text = format!("Health: {}/{} \n", health, MAX_HEALTH);
        let health_section = TextSection::new(
            health_text.clone(),
            TextStyle {
                font: font.clone(),
                font_size,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        );

        let (x, y) = (player.pos_x, player.pos_y);
        let position_text = format!("Position: ({}, {}) \n", x, y);
        let position_section = TextSection::new(
            position_text.clone(),
            TextStyle {
                font: font.clone(),
                font_size,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        );

        let attack_cooldown = player.freeze;
        let attack_text = format!("Attack CD: {}\n", attack_cooldown);
        let attack_section = TextSection::new(
            attack_text.clone(),
            TextStyle {
                font: font.clone(),
                font_size,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        );

        let score = player.score;
        let score_text = format!("Score: {}", score);
        let score_section = TextSection::new(
            score_text.clone(),
            TextStyle {
                font: font.clone(),
                font_size,
                color: Color::srgb(0.9, 0.9, 0.9),
            },
        );

        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([
                    position_section,
                    health_section,
                    attack_section,
                    score_section,
                ])
                // text: Text::from_section(health_text, text_style.clone())
                .with_justify(JustifyText::Center),
                transform: Transform::from_translation(Vec3::new(
                    (pos_index) as f32 * 250.0,
                    50.0,
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

fn update_stats(
    mut query_text: Query<(&mut Text, &ParentStatsId)>,
    query_player_data: Query<(Entity, &PlayerModel), With<PlayerStats>>,
) {
    for (mut text, parent_id) in query_text.iter_mut() {
        for (id, player) in query_player_data.iter() {
            if parent_id.0 == id {
                let (x, y) = (player.pos_x, player.pos_y);
                let position_text = format!("Position: ({}, {}) \n", x, y);
                text.sections[0].value = position_text;

                let health = player.health;
                let health_text = format!("Health: {}/{} \n", health, MAX_HEALTH);
                text.sections[1].value = health_text;

                let cd = player.freeze;
                let cd_text = format!("Attack CD: {}\n", cd);
                text.sections[2].value = cd_text;

                let score = player.score;
                let score_text = format!("Score: {}", score);
                text.sections[3].value = score_text;
            }
        }
    }
}
