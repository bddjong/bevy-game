use bevy::math::{Vec2, Vec3};

pub fn get_world_pos(radius: f32, pos: Vec2, z: f32) -> Vec3 {
    let height = 3_f32.sqrt() * radius;

    let y_offset = if pos.x % 2.0 == 0.0 { 0.0 } else { height / 2.0 };

    let x = pos.x * 3.0 / 2.0 * radius;
    let y = pos.y * height + y_offset;

    Vec3::new(x, z, y)
}