use bevy::prelude::*;

use crate::{
    plugins::dojo_systems::account::PlayerAccount,
    states::GameStates,
    utils::constants::{P1_ADDRESS, P1_PK, P2_ADDRESS, P2_PK},
};

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(
            Update,
            (button_system, handle_button_events).run_if(in_state(GameStates::MainMenu)),
        );
        app.add_systems(
            OnExit(GameStates::MainMenu),
            cleanup_ui.run_if(in_state(GameStates::InGame)),
        );
        app.add_event::<JoinGameEvent>();
        app.add_event::<CreateGameEvent>();
    }
}

#[derive(Component)]
enum ButtonType {
    JoinGame,
    CreateGame,
}

#[derive(Resource)]
pub struct GameMenu {
    ui_entity: Entity,
}

#[derive(Event)]
pub struct JoinGameEvent;

#[derive(Event)]
pub struct CreateGameEvent;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            for (text, button_type) in [
                ("Join Game", ButtonType::JoinGame),
                ("Create Game", ButtonType::CreateGame),
            ] {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(65.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        },
                        button_type,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            text,
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 24.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            }
        })
        .id();

    commands.insert_resource(GameMenu { ui_entity });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonType),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, _button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.35, 0.75, 0.35).into();
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn handle_button_events(
    query: Query<(&Interaction, &ButtonType), (Changed<Interaction>, With<Button>)>,
    mut create_game_writer: EventWriter<CreateGameEvent>,
    mut join_game_writer: EventWriter<JoinGameEvent>,
    mut state: ResMut<NextState<GameStates>>,
    mut commands: Commands,
) {
    for (interaction, button_type) in query.iter() {
        if *interaction == Interaction::Pressed {
            let pk: String;
            let address: String;
            match button_type {
                ButtonType::CreateGame => {
                    _ = create_game_writer.send(CreateGameEvent);
                    pk = P1_PK.to_string();
                    address = P1_ADDRESS.to_string();
                }
                ButtonType::JoinGame => {
                    _ = join_game_writer.send(JoinGameEvent);
                    pk = P2_PK.to_string();
                    address = P2_ADDRESS.to_string();
                }
            };
            commands.insert_resource(PlayerAccount { pk, address });
            state.set(GameStates::InGame);
        }
    }
}

fn cleanup_ui(mut commands: Commands, menu: Option<Res<GameMenu>>) {
    if let Some(menu) = menu {
        commands.entity(menu.ui_entity).despawn_recursive();
    }
}
