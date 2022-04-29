use crate::{
    menu_state::GameState,
};
use bevy::{math::const_vec3, prelude::*};

use super::{walls::{X_OFFSET, Y_OFFSET}, components::{Collider, GameEntity}, };

const PADDLE_WIDTH: f32 = 120.0;
const PADDLE_SIZE: Vec3 = const_vec3!([PADDLE_WIDTH, 20.0, 0.0]);
const PADDLE_SPEED: f32 = 12.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 20.0;
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

#[derive(Component)]
pub struct Paddle;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(render_paddle))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(handle_paddle_move),
            );
    }
}

fn render_paddle(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let arena_height = window.height();
    let paddle_position = -1.0 * (arena_height / 2.0 + Y_OFFSET - GAP_BETWEEN_PADDLE_AND_FLOOR);

    // paddle
    commands
        .spawn()
        .insert(Paddle)
        .insert(Collider)
        .insert(GameEntity)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_position, 1.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        });
}

fn handle_paddle_move(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let window = windows.get_primary().unwrap();
    let arena_width = window.width();
    let mut direction = 0.0;
    let mut player_transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    let new_position = player_transform.translation.x + direction * PADDLE_SPEED;
    let left_bound = calculate_wall_boundary(arena_width) * -1.0 + PADDLE_PADDING;
    let right_bound = calculate_wall_boundary(arena_width) - PADDLE_PADDING;

    player_transform.translation.x = new_position.clamp(left_bound, right_bound);
}

fn calculate_wall_boundary(arena_width: f32) -> f32 {
    arena_width / 2.0 + X_OFFSET - (PADDLE_WIDTH / 2.0)
}
