use crate::game::prelude::GamePlugin;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowMode,
};
use menu_state::*;
use state_plugin::*;

mod game;
mod menu_state;
mod state_plugin;
mod utilities;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
    GameOver,
    Exit,
}

const BACKGROUND_COLOR: Color = Color::rgb(0.20, 0.20, 0.20);
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Breakout!!".to_string(),
            mode: WindowMode::Windowed,
            resizable: false,
            ..Default::default()
        })
        .add_event::<StateChange>()
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(StatePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_state(GameState::MainMenu)
        .run();
}
