use bevy::{
    math::const_vec3,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::{thread_rng, Rng};

use crate::GameState;

use super::components::{Brick, Collider, GameData, GameEntity, FailZone};

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = const_vec3!([0.0, -150.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_SPEED: f32 = 165.0;
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(render_ball))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(check_for_collisions)
                    .with_system(apply_velocity.before(check_for_collisions)),
            );
    }
}

fn render_ball(mut commands: Commands) {
    // randomize initial ball direction
    let mut rng = thread_rng();
    let initial_direction = Vec2::new(rng.gen_range(-0.6..0.6), rng.gen_range(-0.7..-0.1));
    // Ball
    commands
        .spawn()
        .insert(Ball)
        .insert(GameEntity)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                scale: BALL_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(initial_direction.normalize() * BALL_SPEED));
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>, Option<&FailZone>), With<Collider>>,
    mut app_state: ResMut<State<GameState>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    // check collision with walls
    for (collider_entity, transform, maybe_brick, maybe_fail_zone) in collider_query.iter() {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            if maybe_fail_zone.is_some() {
                app_state.set(GameState::GameOver).unwrap();
            } else {
                // Bricks should be despawned and increment the scoreboard on collision
                if maybe_brick.is_some() {
                    game_data.score += 1;
                    commands.entity(collider_entity).despawn();
                }
    
                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;
    
                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }
    
                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }
    
                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}
