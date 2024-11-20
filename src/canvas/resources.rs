use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LastPos(pub Vec2);

#[derive(Resource, Default)]
pub struct LastClicked {
    pub cursor: Vec2,
    pub camera: Vec3,
}

#[derive(Resource, Default)]
pub struct CanvasHandle(pub Handle<Image>);
