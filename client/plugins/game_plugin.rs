use bevy::prelude::*;

use super::{despawn, DisplayQuality, GameState, Volume, MENU_BACKGROUND_COLOR, TEXT_COLOR};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn::<OnGameScreen>);
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display_quality: Res<DisplayQuality>,
    volume: Res<Volume>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(90.0),
                    height: Val::Percent(90.0),
                    // center children
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display two lines of text, the second one with the current settings
                    parent.spawn(
                        TextBundle::from_section(
                            "This will be the game screen eventually",
                            TextStyle {
                                font: font.clone(),
                                font_size: 80.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_sections([
                            TextSection::new(
                                format!("quality: {:?}", *display_quality),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 60.0,
                                    color: TEXT_COLOR,
                                },
                            ),
                            TextSection::new(
                                " - ",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 60.0,
                                    color: TEXT_COLOR,
                                },
                            ),
                            TextSection::new(
                                format!("volume: {:?}", *volume),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 60.0,
                                    color: TEXT_COLOR,
                                },
                            ),
                        ])
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                });
        });
    // Spawn a 5 seconds timer to trigger going back to the menu
    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Endgame);
    }
}
