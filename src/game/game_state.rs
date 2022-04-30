use crate::{menu_state::GameState, utilities::despawn_entities};
use bevy::{app::AppExit, prelude::*};

use super::{
    ball::BallPlugin, bricks::BricksPlugin, components::GameEntity, paddle::PaddlePlugin,
    walls::WallsPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PaddlePlugin)
            .add_plugin(BallPlugin)
            .add_plugin(WallsPlugin)
            .add_plugin(BricksPlugin)
            // setup when entering the state
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game))
            .add_system_set(
                SystemSet::on_exit(GameState::InGame).with_system(despawn_entities::<GameEntity>),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(handle_exit));
    }
}

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

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
