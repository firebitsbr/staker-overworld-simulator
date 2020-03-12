use amethyst::{
    core::{Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::map::MapCoords;
use crate::npc::Movement;

/// System that takes map position and modifies the transform position of the entity accordingly.
pub struct PositionSystem;

impl<'s> System<'s> for PositionSystem {
    type SystemData = (
        ReadStorage<'s, MapCoords>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (map_coords, mut locals): Self::SystemData) {
        for (coords, transform) in (&map_coords, &mut locals).join(){
            transform.set_translation_xyz(coords.x, coords.y, 1.0);
        }
    }
}

/// Modifies map position based on movement rate
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Movement>,
        WriteStorage<'s, MapCoords>,
    );

    fn run(&mut self, (movement, mut map_coords): Self::SystemData) {
        for (movement, map_coords) in (&movement, &mut map_coords).join(){
            map_coords.x += movement.velocity.x;
            map_coords.y += movement.velocity.y;
            map_coords.z += movement.velocity.z;
        }
    }
}