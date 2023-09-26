use bevy::prelude::{Component, Query, Res, Time, Transform};

#[derive(Component)]
pub struct RotateSpeed {
    pub speed: f32,
}

pub fn rotate_system(mut query: Query<(&mut Transform, &RotateSpeed)>, time: Res<Time>) {
    for mut entity in &mut query {
        entity.0.rotate_y(time.delta_seconds() * entity.1.speed);
    }
}