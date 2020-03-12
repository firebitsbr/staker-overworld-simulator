#[allow(unused_imports)]
use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, VecStorage},
    prelude::*,
};

/// Map coordinates, in terms of X/Y cartesian position and an optional Z-level.
#[derive(Debug)]
pub struct MapCoords {
    /// The X position on a map. The origin is on the upper-left corner, increasing
    /// as it moves rightwards.
    pub x: f32,
    /// The Y position on a map. The origin is on the upper-left corner, increasing
    /// as it moves downwards.
    pub y: f32,
    /// A description of the "vertical" position of the coordinates.
    pub z: f32,
}

impl Component for MapCoords {
    type Storage = VecStorage<Self>;
}

/// Indicates the max/min zoom rate for a camera or similar entity
#[derive(Debug)]
pub struct Zoomable {
    /// minimum zoom level
    pub min_zoom: f32,
    /// maximum zoom level
    pub max_zoom: f32,
    /// speed of zoom level changes
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

