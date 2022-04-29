use crate::{menu_state::GameState, utilities::despawn_entities};
use bevy::{app::AppExit, prelude::*};

use super::{ball::BallPlugin, paddle::PaddlePlugin, walls::WallsPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PaddlePlugin)
            .add_plugin(BallPlugin)
            .add_plugin(WallsPlugin)
            // setup when entering the state
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game))
            .add_system_set(
                SystemSet::on_exit(GameState::InGame).with_system(despawn_entities::<GameEntity>),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(handle_exit));
    }
}

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct Collider;

// Defines the amount of time that should elapse between each physics step.
// const TIME_STEP: f32 = 1.0 / 60.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
// const BRICK_SIZE: Vec2 = const_vec2!([100., 30.]);
// These values are exact
// const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
// const GAP_BETWEEN_BRICKS: f32 = 5.0;
// const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
// These values are lower bounds, as the number of bricks is computed
// const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;
// const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // player one score
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(GameEntity);
}

fn handle_exit(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
