use bevy::prelude::*;

use super::{despawn, GameState, MENU_BACKGROUND_COLOR, TEXT_COLOR};

pub struct EndgamePlugin;

impl Plugin for EndgamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Endgame).with_system(splash_setup))
            .add_system_set(SystemSet::on_update(GameState::Endgame).with_system(countdown))
            .add_system_set(
                SystemSet::on_exit(GameState::Endgame).with_system(despawn::<OnEndgameScreen>),
            );
    }
}

#[derive(Component)]
struct OnEndgameScreen;

#[derive(Resource, Deref, DerefMut)]
struct EndgameTimer(Timer);

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
            OnEndgameScreen,
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
                            "This will be the end game screen",
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
                });
        });
    commands.insert_resource(EndgameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<EndgameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu).unwrap();
    }
}
