use bevy::prelude::*;

pub fn despawn_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    println!("despawning entities");
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}
