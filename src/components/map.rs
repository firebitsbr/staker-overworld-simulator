use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::timing::Time,
    core::transform::Transform,
    core::math::geometry::Point3,
    ecs::prelude::{Component, DenseVecStorage, Entity, VecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

/// Map coordinates, in terms of X/Y cartesian position and an optional Z-level.
#[derive(Debug)]
pub struct MapCoords {
    /// The X position on a map. The origin is on the upper-left corner, increasing
    /// as it moves rightwards.
    x: f64,
    /// The Y position on a map. The origin is on the upper-left corner, increasing
    /// as it moves downwards.
    y: f64,
    /// A description of the "vertical" position of the coordinates.
    z: f64,
}

impl Component for MapCoords {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Zoomable {
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_rate: f32,
}

impl Component for Zoomable {
    type Storage = DenseVecStorage<Self>;
}

pub struct Pannable {
    pub pan_rate: f32,
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl Component for Pannable {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct CameraLimits {
    pub min: Point3<f32>,
    pub max: Point3<f32>,
}

impl Component for CameraLimits {
    type Storage = DenseVecStorage<Self>;
}

impl CameraLimits {
    pub fn new() -> Self{
        return CameraLimits{
            min: Point3::<f32>::new(0.0, 0.0, 0.0),
            max: Point3::<f32>::new(0.0, 0.0, 0.0),
        }
    }
}