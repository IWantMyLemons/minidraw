mod canvas;
mod components;
mod systems;

use bevy::prelude::*;
use bevy_image_export::ImageExportPlugin;
use bevy_prototype_lyon::prelude::*;
use canvas::CanvasPlugin;
use systems::{quit_app, setup};

fn main() {
    let export_plugin = ImageExportPlugin::default();
    let export_threads = export_plugin.threads.clone();

    let window = Window {
        decorations: true,
        ..default()
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            }),
            export_plugin,
            ShapePlugin,
            CanvasPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, quit_app)
        .run();

    export_threads.finish();
}
