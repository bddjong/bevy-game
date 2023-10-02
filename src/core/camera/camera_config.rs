use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CameraConfig {
    pub max_distance: f32,
}


impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            max_distance: 100.0
        }
    }
}
