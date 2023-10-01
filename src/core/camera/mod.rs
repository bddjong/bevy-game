use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::Vec3;
use bevy::prelude::{Camera, Camera3dBundle, Commands, OrthographicProjection, Projection, Transform};
use bevy::render::camera::ScalingMode;
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
            far: 50.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            scale: 5.0,
            ..default()
        }),
        camera: Camera {
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 4.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn camera_system() {}
