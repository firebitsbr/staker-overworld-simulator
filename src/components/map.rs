use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Component, VecStorage, DenseVecStorage, Entity},
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
    pub min_zoom: f64,
    pub max_zoom: f64,
    pub current_zoom: f64,
}

impl Component for Zoomable {
    type Storage = DenseVecStorage<Self>;
}