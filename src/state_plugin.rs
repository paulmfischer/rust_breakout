use bevy::{app::AppExit, prelude::*};

use crate::GameState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum StateChange {
    Set(GameState),
    Push(GameState),
    Pop,
    Exit,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_state_change);
    }
}

fn handle_state_change(
    mut event_state_change: EventReader<StateChange>,
    mut exit: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
) {
    for ev in event_state_change.iter() {
        match ev {
            StateChange::Push(state) => app_state.push(*state).unwrap(),
            StateChange::Pop => app_state.pop().unwrap(),
            StateChange::Set(state) => app_state.set(*state).unwrap(),
            StateChange::Exit => exit.send(AppExit),
        }
    }
}
