use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::components::map::{Pannable, CameraLimits};
use crate::utilities::Clampable;

pub struct PanningSystem;

impl<'s> System<'s> for PanningSystem {
    type SystemData = (
        ReadStorage<'s, Pannable>,
        ReadStorage<'s, CameraLimits>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (pan_controls, camera_limits, mut locals, input): Self::SystemData) {
        for (pan_control, limits, local) in (&pan_controls, &camera_limits, &mut locals).join() {
            if let Some(pan_x_value) = input.axis_value("map_pan_x") {
                let current_x = local.translation().x;
                let new_x = (current_x + pan_x_value * pan_control.pan_rate);
                let clamped_x = new_x.clamp_range(limits.min.x, limits.max.x);
                println!("new_x: {}, clamped_x: {}, limits: {}, {}", new_x, clamped_x, limits.min.x, limits.max.x);
                local.set_translation_x(
                    new_x,
                );
            }
            if let Some(pan_y_value) = input.axis_value("map_pan_y") {
                let current_y = local.translation().y;
                let new_y = (current_y + pan_y_value * pan_control.pan_rate);
                let clamped_y = new_y.clamp_range(limits.min.y, limits.max.y);
                println!("new_y: {}, clamped_y: {}, limits: {}, {}", new_y, clamped_y, limits.min.y, limits.max.y);
                local.set_translation_y(
                    clamped_y, 
                );
            }
        }
    }
}