use bevy::app::{App, Plugin, Startup, Update};
use bevy::input::Input;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::Vec3;
use bevy::prelude::{Camera3dBundle, Commands, Component, EventReader, KeyCode, Query, Res, Transform, With};
use bevy::render::camera::{Camera, ScalingMode, Projection, OrthographicProjection};
use bevy::time::Time;
use bevy::utils::default;

use crate::core::config::camera_config::CameraConfig;

pub struct OrthographicCameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for OrthographicCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_orthographic_camera)
            .add_systems(Update, (camera_panning_system, camera_zoom_system));
    }
}

fn setup_orthographic_camera(
    mut commands: Commands,
    camera_config: Res<CameraConfig>,
) {
    commands.spawn(Camera3dBundle {
        projection: Projection::Orthographic(OrthographicProjection {
            near: 0.01,
            far: camera_config.max_distance,
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
    }).insert(MainCamera);
}

fn camera_panning_system(time: Res<Time>, keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<MainCamera>>) {
    for mut transform in query.iter_mut() {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let speed = 50.0;

        let local_forward = transform.forward();
        let right_vector = transform.right();
        let forward_vector = Vec3::new(local_forward.x, 0.0, local_forward.z).normalize();


        if keys.pressed(KeyCode::W) {
            dy += 1.0;
        }
        if keys.pressed(KeyCode::S) {
            dy -= 1.0;
        }
        if keys.pressed(KeyCode::D) {
            dx += 1.0;
        }
        if keys.pressed(KeyCode::A) {
            dx -= 1.0;
        }

        let delta_movement = right_vector * dx + forward_vector * dy;

        if delta_movement.length() > 0.0 {
            transform.translation += delta_movement.normalize() * speed * time.delta_seconds();
        }
    }
}

fn camera_zoom_system(
    mut scroll_event: EventReader<MouseWheel>,
    mut query: Query<&mut Projection, (With<MainCamera>)>,
) {
    for ev in scroll_event.iter() {
        let mut projection = query.single_mut();

        if let Projection::Orthographic(ref mut ortho_proj) = *projection {
            let mut scale = ortho_proj.scale;

            match ev.unit {
                MouseScrollUnit::Line => {
                    scale += ev.y * 0.1;
                }
                MouseScrollUnit::Pixel => {
                    scale += ev.y * 0.01;
                }
            }

            ortho_proj.scale = scale.max(5.0).min(30.0);
        }
    }
}