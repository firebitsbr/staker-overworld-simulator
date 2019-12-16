/// NPC trait
pub trait Npc {
    /// The full display name of this NPC, including any rank or title
    fn get_fullname(&self) -> &str;

    /// The nickname of this NPC , if any
    fn get_nickname(&self) -> &str;

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

/// The various stalker factions that inhabit the zone.
pub enum Faction {
    /// Loners, or free stalkers, are those who chose to remain unaligned
    /// with any of the other factions operating in the zone.
    Loners,
    /// Duty is a paramilitary faction that wages war against the Zone
    /// itself, seeing it as an incubator for untold horrors that would
    /// threaten the entire world if left unchecked.
    Duty,
    /// Clear Sky is an independent scientific and military organisation
    /// focused on researching and understanding the zone without
    /// government oversight or restriction.
    ClearSky,
    /// A fanatical religious cult hostile to all but their own.
    Monolith,
    /// An official body of educated scientists and contracted field 
    /// assistants sponsored by the Ukrainian government, who research
    /// the Zone in order to properly harness its potential for the 
    /// good of mankind.
    Ecologist,
    /// Bandits are Chernobyl's outlaws, who know that while the
    /// Ukrainian government has nominal control over the Zone, it is
    /// in effect a lawless place. Bandits are members of the criminal
    /// underworld who abuse the Zone's anarchic state to hide from the
    /// law and continue their illicit dealings.
    Bandit,
    /// Mercenaries are experienced fighters from all over the world who
    /// offer their services as hired soldiers. It is unknown who their
    /// true clients are, or why they have interests in the Zone.
    Mercenary,
    /// Ukrainian government's internal troops, Spetsnaz detachments and
    /// commisioned stalkers who take a hardline approach to securing
    /// the border between the Zone and the outside world.
    Military,
    /// Freedom are anarchists and daredevils who fight for unregulated
    /// access to the Zone.
    Freedom,
}
