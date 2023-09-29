use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_editor_pls::prelude::*;

use core::{camera::OrthographicCameraPlugin,
           lighting::SunlightPlugin};

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
        .add_plugins((EditorPlugin::default(), FrameTimeDiagnosticsPlugin::default()))
        .add_plugins((OrthographicCameraPlugin, SunlightPlugin))
        .add_systems(Update, close_on_esc)
        .run();
}
