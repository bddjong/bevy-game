mod rotate;
mod camera;

use std::f32::consts::PI;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_editor_pls::prelude::*;

use crate::{
    camera::camera_system::setup_camera,
    rotate::{rotate_system, RotateSpeed}
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.6)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Clanborn"),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }))
        .add_plugins(EditorPlugin::default())

        .add_systems(Startup, (setup_camera, setup_lighting, setup_scene))
        .add_systems(Update, close_on_esc)
        .add_systems(Update, rotate_system)
        .run();
}

fn setup_lighting(
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

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, ) {

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.8 })),
        material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
        transform: Transform::from_xyz(0.0, 0.4, 0.0),
        ..default()
    });

    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.3 })),
        material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
        transform: Transform::from_xyz(0.55, 0.15, 0.2),
        ..default()
    });

    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.6 })),
        material: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
        transform: Transform::from_xyz(-0.5, 0.0, 0.5),
        ..default()
    });

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..default()
    }, RotateSpeed {
        speed: 0.7
    }));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.2, 0.7, 0.3).into()),
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
        ..default()
    }, RotateSpeed {
        speed: 0.2
    }));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.3, 0.3, 0.9).into()),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..default()
    }, RotateSpeed {
        speed: 1.0
    }));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.8, 0.3).into()),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..default()
    }, RotateSpeed {
        speed: 0.5
    }));
}