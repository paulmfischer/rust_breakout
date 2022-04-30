use bevy::prelude::*;

use crate::{menu_state::GameState, utilities::despawn_entities};

use super::components::Collider;

const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const X_OFFSET: f32 = -100.0;
pub const Y_OFFSET: f32 = -15.0;

#[derive(Component)]
pub struct Wall;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(render_walls))
            .add_system_set(
                SystemSet::on_exit(GameState::InGame).with_system(despawn_entities::<Wall>),
            );
    }
}

fn render_walls(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(WallBundle::new(WallLocation::Left, window));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right, window));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top, window));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom, window));
}

// pub fn despawn_walls(commands: Commands, query: Query<Entity, With<Wall>>) {
//     despawn_entities::<Wall>(commands, query);
// }

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self, window: &Window) -> Vec2 {
        let window_height = window.height();
        let window_width = window.width();

        match self {
            WallLocation::Left => Vec2::new((window_width / 2.0 + X_OFFSET) * -1.0, 0.),
            WallLocation::Right => Vec2::new(window_width / 2.0 + X_OFFSET, 0.),
            WallLocation::Bottom => Vec2::new(0., (window_height / 2.0 + Y_OFFSET) * -1.0),
            WallLocation::Top => Vec2::new(0., window_height / 2.0 + Y_OFFSET),
        }
    }

    fn size(&self, window: &Window) -> Vec2 {
        let window_height = window.height();
        let window_width = window.width();
        let arena_side_size = (window_height / 2.0 + Y_OFFSET) * 2.0 + WALL_THICKNESS;
        let arena_top_size = (window_width / 2.0 + X_OFFSET) * 2.0 + WALL_THICKNESS;

        match self {
            WallLocation::Left => Vec2::new(WALL_THICKNESS, arena_side_size),
            WallLocation::Right => Vec2::new(WALL_THICKNESS, arena_side_size),
            WallLocation::Bottom => Vec2::new(arena_top_size, WALL_THICKNESS),
            WallLocation::Top => Vec2::new(arena_top_size, WALL_THICKNESS),
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    wall: Wall,
    collider: Collider,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation, window: &Window) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position(window).extend(0.0),
                    scale: location.size(window).extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            wall: Wall,
            collider: Collider,
        }
    }
}
