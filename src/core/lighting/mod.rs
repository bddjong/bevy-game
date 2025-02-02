use std::f32::consts::PI;

use bevy::app::{App, Plugin, Startup};
use bevy::math::{Quat, Vec3};
use bevy::pbr::{AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{Color, Commands, Res, Transform};
use bevy::utils::default;

use crate::core::config::camera_config::CameraConfig;

pub struct SunlightPlugin;

impl Plugin for SunlightPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_lighting);
    }
}

fn setup_lighting(
    mut commands: Commands,
    camera_config: Res<CameraConfig>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb(1.0, 0.95, 0.96),
        brightness: 0.25,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 33000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 3.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 5.0,
            maximum_distance: camera_config.max_distance,
            ..default()
        }.into(),
        ..default()
    });
}