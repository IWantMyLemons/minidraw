use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use guard_macros::guard;

use super::{components::*, resources::*, *};

#[allow(clippy::too_many_arguments)]
pub fn draw_line(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut last_pos: ResMut<LastPos>,
) {
    guard!(
        !keyboard.pressed(KeyCode::Space),
        mouse.pressed(MouseButton::Left),
        Ok(window) = window.get_single(),
        Ok((camera, camera_transform)) = camera.get_single(),
        Some(cursor_pos) = window.cursor_position(),
        Some(global_cursor_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos),
        last_pos.0.distance(global_cursor_pos) > PEN_SPACING,
    );
    // If this is the first point, set prev to curr, this prevents the line from teleporting
    if mouse.just_pressed(MouseButton::Left) {
        last_pos.0 = global_cursor_pos;
    }

    // Spawn the line
    let in_between = (global_cursor_pos + last_pos.0) / 2.0;
    let delta = global_cursor_pos - last_pos.0;
    commands.spawn((
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
        Line,
    ));

    last_pos.0 = global_cursor_pos;
}

pub fn move_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
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
    mut camera: Query<&mut Transform, With<Camera>>,
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
