pub mod components;
mod resources;
mod systems;

use bevy::prelude::*;
use resources::{LastClicked, LastPos};
use systems::{clear_canvas, draw_line, move_camera, zoom_camera};

const SCROLL_LINE_SCALE: f32 = 0.5;
const SCROLL_PIXEL_SCALE: f32 = 1.0;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastPos>()
            .init_resource::<LastClicked>()
            .add_systems(Update, (draw_line, clear_canvas, move_camera, zoom_camera));
    }
}
