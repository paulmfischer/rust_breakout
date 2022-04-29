use crate::game_state::MenuPlugin;
use bevy::prelude::*;
mod game_state;

const BACKGROUND_COLOR: Color = Color::rgb(0.20, 0.20, 0.20);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}
