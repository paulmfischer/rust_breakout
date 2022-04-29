use bevy::{prelude::*, window::WindowMode};
use game_state::GamePlugin;
use menu_state::*;

mod game_state;
mod menu_state;

const BACKGROUND_COLOR: Color = Color::rgb(0.20, 0.20, 0.20);
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
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
        // .add_state(GameState::MainMenu)
        .run();
}
