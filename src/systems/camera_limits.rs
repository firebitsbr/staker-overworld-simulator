use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{ActiveCamera, Camera, camera::Projection},
    core::math::geometry::Point3,
    window::ScreenDimensions,
};

use crate::components::map::{Pannable, CameraLimits};

/// System that updates CameraLimits component data
pub struct CameraLimitsSystem;

impl<'s> System<'s> for CameraLimitsSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Pannable>,
        WriteStorage<'s, CameraLimits>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (cams, pannables, mut limits, mut transforms, screen): Self::SystemData){
        /// find all pannable cameras that have a CameraLimits component and set the limits
        for (cam, pannable, limit, transform) in (&cams, &pannables, &mut limits, &mut transforms).join() {
            let zlevel = transform.translation().z;
            let proj = cam.projection();
            limit.min = proj.screen_to_world_point(
                Point3::new(0.0, 0.0, zlevel),
                screen.diagonal(),
                transform,
            );

            limit.max = proj.screen_to_world_point(
                Point3::new(screen.width(), screen.height(), zlevel),
                screen.diagonal(),
                transform,
            );
            //println!("{:?}", limit);
        }
    }

}