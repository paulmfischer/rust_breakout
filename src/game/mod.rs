use bevy::prelude::Component;

mod ball;
mod game_state;
mod paddle;
mod walls;

#[derive(Component)]
struct Collider;

pub mod prelude {
    pub use crate::game::game_state::*;
}
