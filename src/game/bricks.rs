use bevy::{math::const_vec2, prelude::*};

use crate::{
    game::components::{Brick, Collider},
    GameState,
};

use super::walls::{X_OFFSET, Y_OFFSET};

const BRICK_COLOR: Color = Color::rgb(0.6, 0.5, 0.4);
const BRICK_SIZE: Vec2 = const_vec2!([100., 30.]);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 300.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;

pub struct BricksPlugin;

impl Plugin for BricksPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(render_bricks));
    }
}

fn render_bricks(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let window_height = window.height();
    let window_width = window.width();

    let arena_width = (window_width / 2.0 + X_OFFSET) * 2.0; // + WALL_THICKNESS;

    let total_width_of_bricks = arena_width - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = (window_height / 2.0 * -1.0) + GAP_BETWEEN_PADDLE_AND_BRICKS;
    let total_height_of_bricks =
        (window_height / 2.0 + Y_OFFSET) - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    let left_edge_of_bricks = 0.0
        // Space taken up by the bricks
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        // Space taken up by the gaps
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            // brick
            commands
                .spawn()
                .insert(Brick)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Collider);
        }
    }
}
