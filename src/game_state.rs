use crate::{menu_state::GameState, utilities::despawn_entities, walls::Y_OFFSET};
use bevy::{
    app::AppExit,
    math::{const_vec2, const_vec3},
    prelude::*,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // setup when entering the state
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .with_system(setup_game)
                    .with_system(crate::walls::render_walls),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::InGame)
                    .with_system(despawn_entities::<GameEntity>)
                    .with_system(crate::walls::despawn_walls),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(handle_exit)
                    .with_system(handle_paddle_move),
            );
    }
}

#[derive(Component)]
struct GameEntity;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Collider;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

// Defines the amount of time that should elapse between each physics step.
// const TIME_STEP: f32 = 1.0 / 60.0;

const PADDLE_WIDTH: f32 = 120.0;
const PADDLE_SIZE: Vec3 = const_vec3!([PADDLE_WIDTH, 20.0, 0.0]);
const PADDLE_SPEED: f32 = 12.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 20.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = const_vec3!([0.0, -50.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
// const BRICK_SIZE: Vec2 = const_vec2!([100., 30.]);
// These values are exact
// const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
// const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
// These values are lower bounds, as the number of bricks is computed
// const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
// const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

// const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
// const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
// const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
// const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let arena_height = window.height();
    let paddle_position = -1.0 * (arena_height / 2.0 + Y_OFFSET - GAP_BETWEEN_PADDLE_AND_FLOOR);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // player one score
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(GameEntity);

    // paddle
    commands
        .spawn()
        .insert(Paddle)
        .insert(Collider)
        .insert(GameEntity)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_position, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        });

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

fn handle_exit(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
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
    arena_width / 2.0 + crate::walls::X_OFFSET - (PADDLE_WIDTH / 2.0)
}
