pub mod components;
pub mod events;
pub mod resources;
mod systems;

use bevy::prelude::*;
use events::DrawLineEvent;
use resources::{LastClicked, LastPos};
use systems::{clear_canvas, draw_line, move_camera, remove_line, stroke_to_line, zoom_camera};

const SCROLL_LINE_SCALE: f32 = 0.5;
const SCROLL_PIXEL_SCALE: f32 = 1.0;

const PEN_SPACING: f32 = 0.01;
const PEN_THICKNESS: f32 = 2.5;
const PEN_COLOR: Color = Color::hsl(0.0, 0.8, 0.7);

const ZOOM_BASE: f32 = 2.0;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastPos>()
            .init_resource::<LastClicked>()
            .add_event::<DrawLineEvent>()
            .add_systems(
                Update,
                (
                    draw_line,
                    clear_canvas,
                    move_camera,
                    zoom_camera,
                    stroke_to_line,
                    remove_line,
                ),
            );
    }
}
