use std::f32::consts::PI;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub fn create_hexagon_plane(radius: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let vertices = (0..6).map(|i| {
        let angle_deg: f32 = 60.0 * i as f32;
        let angle_rad: f32 = PI / 180.0 * angle_deg;
        [radius * angle_rad.cos(), 0.0, radius * angle_rad.sin()]
    }).collect::<Vec<[f32; 3]>>();

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
        vec![[0.0, 1.0, 0.0]; 6],
    );

    return mesh;
}