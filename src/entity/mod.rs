//! Traits and other types that are useful for describing generic entities, like items, 
//! events, NPCs and monsters.

pub mod npc;

/// Map coordinates, in terms of X/Y cartesian position and an optional Z-level.
pub struct MapCoords {
    /// The X position on a map. The origin is on the upper-left corner, increasing 
    /// as it moves rightwards.
    x: u64,
    /// The Y position on a map. The origin is on the upper-left corner, increasing
    /// as it moves downwards.
    y: u64,
    /// A description of the "vertical" position of the coordinates.
    z: ZLevel
}

/// A vertical position that can be used to annotate a map position that is normally 2D.
#[derive(Copy, Clone)]
pub enum ZLevel {
    /// Outside, under the sky. The u64 indicates a "level", where 0 is the ground 
    /// and everything else is progressively higher up.
    Outdoors(u64),
    /// Underground, below the earth. The u64 indicates a "level", where 0 is just-
    /// below ground and everything else is progressively deeper.
    Underground(u64),
    /// Indoors, in some sort of building with "floors". The i64 indicates which
    /// floor, where 0 is the ground floor, negative floors are basements, and
    /// positive floors are above ground. Thus, Indoor(1) corresponds to "the
    /// second floor".
    Indoors(i64),
    /// No valid Z-level descriptor.
    None,
}

impl MapCoords {
    /// Gets the X position of the entity.
    pub fn x(&self)->u64 {
        return self.x;
    }

    /// Gets the Y position of the entity.
    pub fn y(&self)->u64 {
        return self.y;
    }

    /// The Z Level of the entity.
    pub fn z_level(&self) -> ZLevel {
        return self.z;
    }
}