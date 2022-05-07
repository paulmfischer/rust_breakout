use bevy::prelude::*;

use crate::{
    state_plugin::StateChange,
    utilities::{
        self, despawn_entities, MenuButtonAction, MenuEntity, MenuOptions, SelectedOption,
    },
    GameState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(render_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(utilities::menu_interaction)
                    .with_system(select_menu_item),
            )
            .add_system_set(
                SystemSet::on_pause(GameState::MainMenu)
                    .with_system(despawn_entities::<MenuEntity>),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu).with_system(despawn_entities::<MenuEntity>),
            );
    }
}

fn render_menu(commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    utilities::render_menu(
        commands,
        asset_server,
        windows,
        MenuOptions {
            title: "Breakout!!",
            play_text: "Start",
        },
    );
}

fn select_menu_item(
    keyboard_input: ResMut<Input<KeyCode>>,
    selected_option_query: Query<&MenuButtonAction, With<SelectedOption>>,
    mut event_state_change: EventWriter<StateChange>,
) {
    let menu_action = selected_option_query.single();

    if keyboard_input.just_pressed(KeyCode::Return) {
        match menu_action {
            MenuButtonAction::Play => {
                event_state_change.send(StateChange::Set(GameState::InGame));
            }
            MenuButtonAction::Quit => {
                event_state_change.send(StateChange::Exit);
            }
        }
    }
}
