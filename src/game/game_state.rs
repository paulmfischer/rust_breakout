use crate::{utilities::despawn_entities, GameState};
use bevy::prelude::*;

use super::{
    ball::BallPlugin,
    bricks::BricksPlugin,
    components::{GameData, GameEntity, Scoreboard},
    paddle::PaddlePlugin,
    pause_state::PausePlugin,
    walls::WallsPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameData { score: 0 })
            .add_plugin(PaddlePlugin)
            .add_plugin(BallPlugin)
            .add_plugin(WallsPlugin)
            .add_plugin(BricksPlugin)
            .add_plugin(PausePlugin)
            // setup when entering the state
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game))
            .add_system_set(
                SystemSet::on_exit(GameState::InGame).with_system(despawn_entities::<GameEntity>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(handle_pause_game)
                    .with_system(update_score),
            );
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
        .insert(Scoreboard)
        .insert(GameEntity);
}

fn handle_pause_game(keyboard_input: Res<Input<KeyCode>>, mut app_state: ResMut<State<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state.push(GameState::Paused).unwrap();
    }
}

fn update_score(
    game_data: Res<GameData>,
    mut scoreboard_query: Query<&mut Text, With<Scoreboard>>,
) {
    let mut scoreboard_text = scoreboard_query.single_mut();
    scoreboard_text.sections[0].value = format!("{}", game_data.score);
}
