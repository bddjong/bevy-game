use bevy::app::{App, Plugin, Startup, Update};

use camera_system::{camera_system, setup_ortho_camera};

mod camera_system;

pub struct OrthographicCameraPlugin;

impl Plugin for OrthographicCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ortho_camera)
            .add_systems(Update, camera_system);
    }
}