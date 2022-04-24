#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}

// pub fn setup() {

// }

use bevy::prelude::{App, Commands, Plugin, SystemSet};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // add the app state type
        .add_state(AppState::MainMenu)
        // systems to run only in the main menu
        // .add_system_set(
        //     SystemSet::on_update(AppState::MainMenu)
        //         .with_system(handle_ui_buttons)
        // )
        // setup when entering the state
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_menu)
        )
        // // cleanup when exiting the state
        // .add_system_set(
        //     SystemSet::on_exit(AppState::MainMenu)
        //         .with_system(close_menu)
        // )
        ;
    }
}

fn setup_menu(mut commands: Commands) {}
