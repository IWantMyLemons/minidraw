pub mod components;
pub mod events;
pub mod resources;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::{
    draw::Stroke,
    prelude::{LineCap, LineJoin},
};
use events::DrawLineEvent;
use resources::{CanvasHandle, LastClicked, LastPos};
use systems::*;

const SCROLL_LINE_SCALE: f32 = 0.5;
const SCROLL_PIXEL_SCALE: f32 = 1.0;

const PEN_SPACING: f32 = 0.01;
const PEN_THICKNESS: f32 = 2.5;
const PEN_COLOR: Color = Color::hsl(0.0, 0.8, 0.7);

fn get_stroke_settings() -> Stroke {
    let mut s = Stroke::new(PEN_COLOR, 2.0 * PEN_THICKNESS);
    s.options.start_cap = LineCap::Round;
    s.options.end_cap = LineCap::Round;
    s.options.line_join = LineJoin::Round;
    s
}

const ZOOM_BASE: f32 = 2.0;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastPos>()
            .init_resource::<LastClicked>()
            .init_resource::<CanvasHandle>()
            .add_event::<DrawLineEvent>()
            .add_systems(
                Startup,
                (
                    make_empty_canvas,
                    spawn_render_camera.after(make_empty_canvas),
                ),
            )
            .add_systems(
                Update,
                (
                    draw_line,
                    clear_canvas,
                    move_camera,
                    zoom_camera,
                    stroke_to_line,
                    remove_line,
                    save_file,
                ),
            );
    }
}
