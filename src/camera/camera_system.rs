use bevy::math::Vec3;
use bevy::prelude::{Camera3dBundle, Commands, Transform};
use bevy::render::camera::{Camera, OrthographicProjection, Projection, ScalingMode};
use bevy::utils::default;

pub fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        projection: Projection::Orthographic(OrthographicProjection {
            near: 0.01,
            far: 50.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            scale: 5.0,
            ..default()
        }),
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 4.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_system() {}
