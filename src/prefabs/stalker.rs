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
pub struct Stalker {
    player: Named,
    position: MapCoords,
    movement: Movement,
}