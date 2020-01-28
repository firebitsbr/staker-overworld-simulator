//! Components useful for defining Non-Player Characters, like
//! monsters, other human characters.

use amethyst::{
    assets::{PrefabData},
    ecs::{Component, VecStorage, DenseVecStorage, Entity, NullStorage, WriteStorage},
    derive::PrefabData,
    core::math::Vector3,
    Error,
};

/// Component for entities that have names used for displays
#[derive(Debug)]
pub struct Named {
    /// Full name, including any formal title
    full_name: String,
    /// Nickname, if any
    nickname: Option<String>,
}

impl Component for Named {
    type Storage = VecStorage<Self>;
}

/// Component used for entities that belong to a certain faction or
/// have faction relationships.
#[derive(Debug)]
pub struct Faction {
    faction_name: String,
}

impl Component for Faction {
    type Storage = VecStorage<Self>;
}

/// The relationship status of a given faction to another.
pub enum FactionRelation {
    /// Hostile relationship, will kill on sight.
    Hostile,
    /// Neutral relationship, will ignore, won't assist in fights.
    Neutral,
    /// Friendly relationship, will assist in fights, will never attack.
    Friendly,
}

#[derive(Clone, smart_default::SmartDefault, Debug, PrefabData)]
#[prefab(Component)]
pub struct Movement {
    #[default(Vector3::zeros())]
    pub velocity: Vector3<f32>,
    pub max_movement_speed: f32,
}
impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}