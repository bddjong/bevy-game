use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_editor_pls::prelude::*;

use core::{
    camera::OrthographicCameraPlugin,
    lighting::SunlightPlugin,
};
use core::mesh::hexagon;
use hexagon::create_hexagon_plane;

mod core;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.6)))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Bevy Demo"),
                    ..default()
                }),
                ..default()
            }))
        .add_plugins((EditorPlugin::default(), LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
        .add_plugins((OrthographicCameraPlugin, SunlightPlugin))
        .add_systems(Startup, create_world)
        .add_systems(Update, close_on_esc)
        .run();
}

fn create_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let radius = 1.0;
    let height = 3_f32.sqrt() * radius;
    let mesh_handle = meshes.add(create_hexagon_plane(radius));
    let material_handle = materials.add(Color::rgb(1.0, 0.5, 0.5).into());
    let material_handle_b = materials.add(Color::rgb(0.5, 1.0, 0.5).into());

    commands.spawn(PbrBundle {
        mesh: mesh_handle.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: mesh_handle.clone(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, height),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: mesh_handle.clone(),
        material: material_handle_b.clone(),
        transform: Transform::from_xyz(3.0 / 2.0 * radius, 0.0, height / 2.0),
        ..Default::default()
    });
}