//! Components that are useful for all types of entities
pub mod npc;

use specs::{
    Component, VecStorage
};

/// Map coordinates, in terms of X/Y cartesian position and an optional Z-level.
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct MapCoords {
    /// The X position on a map. The origin is on the upper-left corner, increasing 
    /// as it moves rightwards.
    x: f64,
    /// The Y position on a map. The origin is on the upper-left corner, increasing
    /// as it moves downwards.
    y: f64,
    /// A description of the "vertical" position of the coordinates.
    z: f64
}