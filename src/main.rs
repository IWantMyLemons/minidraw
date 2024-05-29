mod canvas;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use canvas::CanvasPlugin;
use systems::{quit_app, setup};

fn main() {
    let window = Window {
        decorations: true,
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_plugins(CanvasPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, quit_app)
        .run();
}
