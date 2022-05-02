use bevy::prelude::Component;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Scoreboard;

pub struct GameData {
    pub score: i32,
}
