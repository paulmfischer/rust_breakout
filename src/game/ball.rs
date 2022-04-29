use bevy::{
    math::{const_vec2, const_vec3},
    prelude::*,
};

use crate::menu_state::GameState;

use super::prelude::GameEntity;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = const_vec3!([0.0, -50.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_SPEED: f32 = 400.0;
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const INITIAL_BALL_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(render_ball));
    }
}

fn render_ball(mut commands: Commands) {
    // Ball
    commands
        .spawn()
        .insert(Ball)
        .insert(GameEntity)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));
}
