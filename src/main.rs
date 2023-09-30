use std::f32::consts::PI;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::window::close_on_esc;
use bevy_editor_pls::prelude::*;

use core::{
    camera::OrthographicCameraPlugin,
    lighting::SunlightPlugin,
};

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
        .add_systems(Startup, spawn_procedural_cube)
        .add_systems(Startup, test)
        .add_systems(Update, close_on_esc)
        .run();
}

fn test(meshes: Res<Assets<Mesh>>, materials: Res<Assets<StandardMaterial>>) {
    println!("{}", meshes.len());
    println!("{}", materials.len());
}

fn spawn_procedural_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 1.0; // Assuming the hexagon has a radius of 1.0. Adjust if needed.
    let hex_mesh = create_hex(radius);
    let hex_mesh_handle = meshes.add(hex_mesh);
    let material_red = materials.add(Color::rgb(1.0, 0.2, 0.2).into());
    let material_white = materials.add(Color::rgb(1.0, 1.0, 1.0).into());

    let height = radius * f32::sqrt(3.0);  // height of the equilateral triangle formed by 2 consecutive vertices of the hexagon

    // Fourth hexagon below and to the right of the first (accounting for the odd-r offset)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: material_white.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn create_hex(radius: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // Calculate the vertices for the hexagon
    let vertices = (0..6).map(|i| {
        let angle_deg: f32 = 60.0 * i as f32;
        let angle_rad: f32 = PI / 180.0 * angle_deg;
        [radius * angle_rad.cos(), 0.0, radius * angle_rad.sin()]
    }).collect::<Vec<[f32; 3]>>();

    // Set indices to create triangles
    let indices = vec![
        0, 5, 1,
        1, 5, 4,
        1, 4, 2,
        2, 4, 3,
    ];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 1.0, 0.0]; 6],  // Assuming you have 6 vertices for the hexagon
    );

    return mesh;
}