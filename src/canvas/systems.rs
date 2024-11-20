use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use bevy_image_export::{ImageExportBundle, ImageExportSettings, ImageExportSource};
use bevy_prototype_lyon::prelude::*;
use guard_macros::guard;

use crate::components::CanvasCamera;

use super::{components::*, events::*, resources::*, *};

#[allow(clippy::too_many_arguments)]
pub fn draw_line(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<CanvasCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut last_pos: ResMut<LastPos>,
    mut draw_writer: EventWriter<DrawLineEvent>,
) {
    guard!(!keyboard.pressed(KeyCode::Space));

    if mouse.just_released(MouseButton::Left) {
        draw_writer.send(DrawLineEvent);
    }

    guard!(
        mouse.pressed(MouseButton::Left),
        Ok(window) = window.get_single(),
        Ok((camera, camera_transform)) = camera.get_single(),
        Some(cursor_pos) = window.cursor_position(),
        Some(global_cursor_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos),
        last_pos.0.distance(global_cursor_pos)
            > camera_transform.compute_transform().scale.x * PEN_SPACING,
    );

    // If this is the first point, set prev to curr, this prevents the line from teleporting
    if mouse.just_pressed(MouseButton::Left) {
        last_pos.0 = global_cursor_pos;
    }

    commands.spawn(Point(global_cursor_pos));

    // Spawn the line
    let in_between = (global_cursor_pos + last_pos.0) / 2.0;
    let delta = global_cursor_pos - last_pos.0;
    commands.spawn((
        PenStroke,
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(PEN_THICKNESS, delta.length()))),
            material: materials.add(PEN_COLOR),
            transform: Transform {
                translation: Vec3 {
                    x: in_between.x,
                    y: in_between.y,
                    z: 0.0,
                },
                rotation: Quat::from_rotation_z(delta.to_angle() + PI / 2.0),
                ..default()
            },
            ..default()
        },
    ));

    last_pos.0 = global_cursor_pos;
}

pub fn move_camera(
    mut camera: Query<&mut Transform, With<CanvasCamera>>,
    window: Query<&Window, With<PrimaryWindow>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut last_clicked: ResMut<LastClicked>,
) {
    guard!(
        Ok(mut transform) = camera.get_single_mut(),
        Ok(window) = window.get_single(),
        Some(cursor_pos) = window.cursor_position(),
    );

    if (keyboard.pressed(KeyCode::Space) && mouse.just_pressed(MouseButton::Left))
        || (keyboard.just_pressed(KeyCode::Space) && mouse.pressed(MouseButton::Left))
    {
        last_clicked.cursor = cursor_pos;
        last_clicked.camera = transform.translation;
    }

    if keyboard.pressed(KeyCode::Space) && mouse.pressed(MouseButton::Left) {
        let delta = cursor_pos - last_clicked.cursor;
        transform.translation = last_clicked.camera
            + Vec3 {
                x: -delta.x,
                y: delta.y,
                z: 0.0,
            } * transform.scale;
    }
}

pub fn zoom_camera(
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<&mut Transform, With<CanvasCamera>>,
    mut scroll_reader: EventReader<MouseWheel>,
) {
    guard!(
        Ok(mut transform) = camera.get_single_mut(),
        Ok(window) = window.get_single(),
        Some(cursor_pos) = window.cursor_position(),
    );

    for scroll in scroll_reader.read() {
        let distance = match scroll.unit {
            MouseScrollUnit::Line => -scroll.y * SCROLL_LINE_SCALE,
            MouseScrollUnit::Pixel => -scroll.y * SCROLL_PIXEL_SCALE,
        };

        let delta = cursor_pos - Vec2::new(window.width(), window.height()) / 2.0;
        let old_scale = transform.scale.x;
        transform.scale *= ZOOM_BASE.powf(distance);
        let new_scale = transform.scale.x;

        transform.translation += Vec3::new(-delta.x, delta.y, 0.0) * (new_scale - old_scale);
    }
}

pub fn clear_canvas(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    points: Query<Entity, With<Point>>,
    lines: Query<Entity, With<Line>>,
) {
    guard!(keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyX));

    for entity in points.iter() {
        commands.entity(entity).despawn();
    }
    for entity in lines.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn stroke_to_line(
    mut commands: Commands,
    mut draw_reader: EventReader<DrawLineEvent>,
    points: Query<(Entity, &Point)>,
) {
    for _ in draw_reader.read() {
        let mut path_builder = PathBuilder::new();

        for (entity, point) in points.iter() {
            path_builder.line_to(point.0);
            commands.entity(entity).despawn();
        }

        let path = path_builder.build();

        commands.spawn((
            Line,
            ShapeBundle { path, ..default() },
            get_stroke_settings(),
        ));
    }
}

pub fn remove_line(
    mut commands: Commands,
    mut draw_reader: EventReader<DrawLineEvent>,
    strokes: Query<Entity, With<PenStroke>>,
) {
    for _ in draw_reader.read() {
        for entity in strokes.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn make_empty_canvas(mut images: ResMut<Assets<Image>>, mut canvas: ResMut<CanvasHandle>) {
    let size = Extent3d {
        width: 2048,
        height: 2048,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    canvas.0 = image_handle;
}

pub fn spawn_render_camera(mut commands: Commands, canvas: Res<CanvasHandle>) {
    commands.spawn((
        RenderCamera,
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: canvas.0.clone().into(),
                ..default()
            },
            ..default()
        },
    ));

    // commands.spawn(SpriteBundle {
    //     texture: canvas.0.clone(),
    //     transform: Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
    //     ..default()
    // });
}

pub fn save_file(
    mut commands: Commands,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
    image: Res<CanvasHandle>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    guard!(keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyS));

    commands.spawn(ImageExportBundle {
        source: export_sources.add(image.0.clone()),
        settings: ImageExportSettings {
            output_dir: "out".into(),
            extension: "png".into(),
        },
    });
}
