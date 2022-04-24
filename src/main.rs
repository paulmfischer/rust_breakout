use crate::game_state::MenuPlugin;
use bevy::{prelude::App, DefaultPlugins};
mod game_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .run();
}
