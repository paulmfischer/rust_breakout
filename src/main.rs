use crate::game::prelude::GamePlugin;
use bevy::{prelude::*, window::WindowMode};
use menu_state::*;

mod game;
mod menu_state;
mod utilities;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
}

const BACKGROUND_COLOR: Color = Color::rgb(0.20, 0.20, 0.20);
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Breakout!!".to_string(),
            // vsync: false,
            mode: WindowMode::Windowed,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_state(GameState::MainMenu)
        .run();
}
