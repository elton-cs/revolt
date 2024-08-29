use bevy::prelude::*;

pub struct GameMenuPlugin;
impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, (button_system, handle_button_events))
           .add_event::<JoinGameEvent>()
           .add_event::<CreateGameEvent>();
    }
}

#[derive(Component)]
enum ButtonType {
    JoinGame,
    CreateGame,
}

#[derive(Event)]
pub struct JoinGameEvent;

#[derive(Event)]
pub struct CreateGameEvent;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
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
            for (text, button_type) in [("Join Game", ButtonType::JoinGame), ("Create Game", ButtonType::CreateGame)] {
                parent.spawn((
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
                )).with_children(|parent| {
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
        });
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
) {
    for (interaction, button_type) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button_type {
                ButtonType::CreateGame => _ = create_game_writer.send(CreateGameEvent),
                ButtonType::JoinGame => _ = join_game_writer.send(JoinGameEvent),
            };
        }
    }
}