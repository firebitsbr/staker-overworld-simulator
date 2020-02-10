use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::map::Zoomable;
use crate::utilities::Clampable;

pub struct ZoomingSystem;

impl<'s> System<'s> for ZoomingSystem {
    type SystemData = (
        ReadStorage<'s, Zoomable>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (zoom_controls, mut locals, input): Self::SystemData) {
        for (zoom_control, local) in (&zoom_controls, &mut locals).join() {
            if let Some(zoom_value) = input.axis_value("map_zoom") {
                let current_zoom = local.translation().z;
                let new_zoom = (current_zoom + zoom_value * zoom_control.zoom_rate);
                let clamped_zoom =
                    new_zoom.clamp_range(zoom_control.min_zoom, zoom_control.max_zoom);
                local.set_translation_z(clamped_zoom);
            }
        }
    }
}
