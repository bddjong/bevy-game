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

use crate::hexmap::get_world_pos;

mod core;
mod hexmap;

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
    let mesh_handle = meshes.add(create_hexagon_plane(radius));
    let material_handle_red = materials.add(Color::rgb(1.0, 0.5, 0.5).into());
    let material_handle_green = materials.add(Color::rgb(0.5, 1.0, 0.5).into());

    for y in 0..10 {
        for x in 0..10 {
            commands.spawn(PbrBundle {
                mesh: mesh_handle.clone(),
                material: material_handle_red.clone(),
                transform: Transform::from_translation(
                    get_world_pos(radius, Vec2::new(x as f32, y as f32), 0.0)),
                ..Default::default()
            });
        }
    }
}