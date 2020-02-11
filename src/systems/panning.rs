use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

use crate::components::map::Pannable;
use crate::utilities::Clampable;

pub struct PanningSystem;

impl PanningSystem {}

impl<'s> System<'s> for PanningSystem {
    type SystemData = (
        ReadStorage<'s, Pannable>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (pan_controls, mut locals, input, time): Self::SystemData) {
        for (pan_control, local) in (&pan_controls, &mut locals).join() {
            let delta_time = time.delta_seconds();
            let mut pan_x_value = input.axis_value("map_pan_x").unwrap_or(0.0);
            let mut pan_y_value = input.axis_value("map_pan_y").unwrap_or(0.0);
            if input.action_is_down("enable_joystick_pan").unwrap_or(false) {
                pan_x_value = input.axis_value("map_joystick_pan_x").unwrap_or(0.0);
                // Y axis invert is more natural?
                pan_y_value = -1.0 * input.axis_value("map_joystick_pan_y").unwrap_or(0.0);
            }
            let current_x = local.translation().x;
            let new_x = (current_x + pan_x_value * pan_control.pan_rate);
            let clamped_x = new_x.clamp_range(pan_control.min_x, pan_control.max_x);
            local.set_translation_x(clamped_x);

            let current_y = local.translation().y;
            let new_y = (current_y + pan_y_value * pan_control.pan_rate);
            let clamped_y = new_y.clamp_range(pan_control.min_y, pan_control.max_y);
            local.set_translation_y(clamped_y);
        }
    }
}
