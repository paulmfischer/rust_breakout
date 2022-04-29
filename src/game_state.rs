use crate::menu_state::GameState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // add the app state type
            .add_state(GameState::InGame)
            // systems to run only in the main menu
            // setup when entering the state
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game))
        // .add_system_set(
        //     SystemSet::on_update(AppState::MainMenu)
        //         .with_system(menu_interaction)
        //         .with_system(select_menu_item),
        // )
        // cleanup when exiting the state
        // .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(close_menu))
        ;
    }
}

#[derive(Component)]
struct GameEntity;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
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
