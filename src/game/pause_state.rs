use bevy::{app::AppExit, prelude::*};

use crate::{
    utilities::{
        self, despawn_entities, MenuButtonAction, MenuEntity, MenuOptions, SelectedOption,
    },
    GameState,
};

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Paused).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
                    .with_system(utilities::menu_interaction)
                    .with_system(select_menu_item),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Paused).with_system(despawn_entities::<MenuEntity>),
            );
    }
}

fn setup_menu(commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    utilities::setup_menu(
        commands,
        asset_server,
        windows,
        MenuOptions {
            title: "Paused",
            play_text: "Continue",
        },
    );
}

fn select_menu_item(
    keyboard_input: Res<Input<KeyCode>>,
    selected_option_query: Query<&MenuButtonAction, With<SelectedOption>>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
) {
    let menu_action = selected_option_query.single();

    if keyboard_input.just_pressed(KeyCode::Return) {
        match menu_action {
            MenuButtonAction::Play => {
                app_state.pop().unwrap();
            }
            MenuButtonAction::Quit => {
                exit.send(AppExit);
            }
        }
    }
}
