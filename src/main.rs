#![deny(missing_docs)]

//! Hello world

mod utilities;
#[allow(unused_imports)]
use crate::utilities::*;

mod components;
#[allow(unused_imports)]
use crate::components::*;

mod backbone;
use backbone::simulator::SimulatorRunState;

mod systems;
use crate::systems::*;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir, auto_fov::AutoFovSystem},
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // display config
    let display_config_path = app_root.join("config").join("display.ron");

    // assets config
    let assets_dir = app_root.join("assets");
    // keyboard bindings
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .unwrap()
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
            // plugin for rendering GUI elements, like the score
            // .with_plugin(RenderUi::default()),
        )?
        // Add the inpput bundle which handles keyboard / mouse input
        .with_bundle(input_bundle)?
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with(AutoFovSystem::new(), "fov_system", &[])
        .with(ZoomingSystem, "zooming_system", &["input_system"])
        .with(PanningSystem, "panning_system", &["input_system"])
        .with(MovementSystem, "movement_system", &[])
        .with(PositionSystem, "position_system", &["movement_system"]);

    // let mut world = World::new();
    let mut game = Application::new(assets_dir, SimulatorRunState::default(), game_data)?;
    game.run();

    Ok(())
}
