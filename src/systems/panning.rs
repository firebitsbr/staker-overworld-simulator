use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::map::{Pannable};
use crate::utilities::Clampable;

pub struct PanningSystem;

impl<'s> System<'s> for PanningSystem {
    type SystemData = (
        ReadStorage<'s, Pannable>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (pan_controls, mut locals, input): Self::SystemData) {
        for (pan_control, local) in (&pan_controls, &mut locals).join() {
            if let Some(pan_x_value) = input.axis_value("map_pan_x") {
                let current_x = local.translation().x;
                let new_x = (current_x + pan_x_value * pan_control.pan_rate);
                let clamped_x = new_x.clamp_range(pan_control.min_x, pan_control.max_y);
                local.set_translation_x(
                    clamped_x,
                );
            }
            if let Some(pan_y_value) = input.axis_value("map_pan_y") {
                let current_y = local.translation().y;
                let new_y = (current_y + pan_y_value * pan_control.pan_rate);
                let clamped_y = new_y.clamp_range(pan_control.min_y, pan_control.max_y);
                local.set_translation_y(
                    clamped_y,
                );
            }
        }
    }
}
