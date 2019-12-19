//! Traits and structures that are useful for defining Non-Player Characters, like
//! monsters, other human characters.

/// NPC trait
pub trait Npc {
    /// The full display name of this NPC, including any rank or title
    fn get_fullname(&self) -> &str;

    /// The nickname of this NPC, if any
    fn get_nickname(&self) -> &str {
        self.get_fullname()
    }

    /// The faction that this  belongs to.
    fn get_faction(&self) -> &dyn NpcFaction;
}

/// Interface for retrieving faction info
pub trait NpcFaction {
    /// The display name of the faction.
    fn get_name(&self) -> &str;

    /// Get the current relationship of this faction to another.
    fn get_relation(&self, other: &dyn NpcFaction) -> FactionRelation;
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