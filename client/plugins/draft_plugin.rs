use bevy::prelude::*;

use super::{button_system, despawn, GameState, MENU_BACKGROUND_COLOR, NORMAL_BUTTON, TEXT_COLOR, InteractedButton};

pub struct DraftPlugin;

impl Plugin for DraftPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DraftTimer(Timer::from_seconds(10.0, TimerMode::Once)))
            .add_system(splash_setup.in_schedule(OnEnter(GameState::Draft)))
            .add_system(countdown.in_set(OnUpdate(GameState::Draft)))
            .add_system(countdown_update.in_set(OnUpdate(GameState::Draft)))
            .add_system(draft_actions.in_set(OnUpdate(GameState::Draft)))
            .add_system(button_system.in_set(OnUpdate(GameState::Draft)))
            .add_system(despawn::<OnDraftScreen>.in_schedule(OnExit(GameState::Draft)));
    }
}

#[derive(Component)]
enum DraftButtonActions {
    Ready,
}

#[derive(Component)]
struct OnDraftScreen;

#[derive(Component)]
struct RemainingTime;

#[derive(Resource, Deref, DerefMut)]
struct DraftTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnDraftScreen,
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
                    parent.spawn(
                        TextBundle::from_section(
                            "This will be the draft screen",
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
                        TextBundle::from_section(
                            "You have ten seconds to pretend to be drafting cards",
                            TextStyle {
                                font: font.clone(),
                                font_size: 20.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(25.0)),
                            ..default()
                        }),
                    );
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                                    margin: UiRect::all(Val::Px(20.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            DraftButtonActions::Ready,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Ready Up",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    color: TEXT_COLOR,
                                },
                            ));
                        });
                    parent.spawn((
                        TextBundle::from_section(
                            "remaining time: 10",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: TEXT_COLOR,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::right(Val::Px(10.0)),
                            ..default()
                        }),
                        RemainingTime,
                        OnDraftScreen,
                    ));
                });
        });
    commands.insert_resource(DraftTimer(Timer::from_seconds(10.0, TimerMode::Once)));
}

fn countdown_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    timer: ResMut<DraftTimer>,
    to_despawn: Query<Entity, With<RemainingTime>>
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
    let time_remaining = timer.remaining_secs().round().to_string();
    commands.spawn((
        TextBundle::from_section(
            "remaining time: ".to_owned() + &time_remaining,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: TEXT_COLOR,
            },
        )
        .with_style(Style {
            margin: UiRect::right(Val::Px(10.0)),
            ..default()
        }),
        RemainingTime,
        OnDraftScreen,
    ));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<DraftTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Game);
    }
}

type InteractedAction<'a> = (&'a Interaction, &'a DraftButtonActions);

fn draft_actions(
    interaction_query: Query<
        InteractedAction<'_>,
        InteractedButton,
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match button_action {
                DraftButtonActions::Ready => {
                    game_state.set(GameState::Game);
                }
            }
        }
    }
}
