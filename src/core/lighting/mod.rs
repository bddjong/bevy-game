use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};
use bevy::pbr::{AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{Color, Commands, Transform};
use bevy::utils::default;

pub fn setup_lighting(
    mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb(1.0, 0.95, 0.96),
        brightness: 0.20,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 30000.0,
            shadow_normal_bias: 1.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 3.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 5.0,
            maximum_distance: 50.0,
            ..default()
        }.into(),
        ..default()
    });
}