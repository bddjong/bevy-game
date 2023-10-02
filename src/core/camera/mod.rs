use bevy::app::{App, Plugin, Startup, Update};
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::{Camera, Camera3dBundle, Commands, KeyCode, OrthographicProjection, Projection, Query, Res, Transform, With};
use bevy::render::camera::ScalingMode;
use bevy::time::Time;
use bevy::utils::default;

pub struct OrthographicCameraPlugin;

impl Plugin for OrthographicCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_orthographic_camera)
            .add_systems(Update, camera_system);
    }
}

fn setup_orthographic_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        projection: Projection::Orthographic(OrthographicProjection {
            near: 0.01,
            far: 100.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            scale: 15.0,
            ..default()
        }),
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(-30.0, 30.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_system(time: Res<Time>, keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Camera>>) {
    for mut transform in query.iter_mut() {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let speed = 50.0;

        if keys.pressed(KeyCode::W) {
            dy -= 1.0;
        }
        if keys.pressed(KeyCode::S) {
            dy += 1.0;
        }
        if keys.pressed(KeyCode::D) {
            dx += 1.0;
        }
        if keys.pressed(KeyCode::A) {
            dx -= 1.0;
        }

        let delta_movement = Vec3::new(dx, 0.0, dy);

        if delta_movement.length() > 0.0 {
            transform.translation += delta_movement.normalize() * speed * time.delta_seconds();
        }
    }
}
