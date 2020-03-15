use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::Entity,
    Error,
};
use serde::{Deserialize, Serialize};

use crate::components::map::MapCoords;
use crate::components::npc::{Named, Movement};

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct Stalker {
    name: Named,
    position: MapCoords,
    movement: Movement,
}