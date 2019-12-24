//! Components useful for defining Non-Player Characters, like
//! monsters, other human characters.

use specs::{
    Component, VecStorage
};

/// Component for entities that have names used for displays
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Named {
    /// Full name, including any formal title
    full_name: String,
    /// Nickname, if any
    nickname: Option<String>,
}

/// Component used for entities that belong to a certain faction or
/// have faction relationships.
#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Faction {
    faction_name: String,
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