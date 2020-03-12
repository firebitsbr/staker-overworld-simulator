use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{
        Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    input::{VirtualKeyCode, is_key_down},
    utils::auto_fov::AutoFov,
};

use crate::components::map::{Pannable, Zoomable};

/// default size of the map
/// TODO: make this a runtime resource
pub const MAP_HEIGHT: f32 = 2048.0;
pub const MAP_WIDTH: f32 = 1024.0;

/// Struct for primary simulation application state, such as when
/// the simulation is active and interactive
#[derive(Default)]
pub struct SimulatorRunState {
    map_sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for SimulatorRunState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.map_sprite_sheet_handle.replace(load_map_sprite(world));

        world.register::<Zoomable>();

        initialize_map(world, self.map_sprite_sheet_handle.clone().unwrap());
        initialize_camera(world);
        println!("Simulation started.");
    }

    fn on_pause(&mut self,_data: StateData<'_, GameData<'_, '_>>) {
        println!("Pause simulation run state.");
    }

    fn on_resume(&mut self,_data: StateData<'_, GameData<'_, '_>>) {
        println!("Resuming simulation run state.");
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                return Trans::Push(Box::new(SimulatorInitializingState));
            }
        }
        Trans::None
    }
}

/// Struct for the beginning of the simulation, where the first entities need to be spawned.
/// This state should typically just push onto the running state whenever the simulation
/// needs to be initialized.
#[derive(Default)]
pub struct SimulatorInitializingState;

impl SimpleState for SimulatorInitializingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Initializing simulation prefab entities.");
        let world = data.world;
        initialize_example_entities(world);
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}


/// Loads the map texture.
/// TODO: Make this configurable at runtime.
fn load_map_sprite(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        // load and return the texture handle
        loader.load(
            "texture/stalkerAnomalyMap.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/stalkerAnomalyMap.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_map(world: &mut World, map_sprite_sheet_handle: Handle<SpriteSheet>) {
    // Create the translation.
    let mut local_transform = Transform::default();

    // Assign the sprite for the map
    let sprite_render = SpriteRender {
        sprite_sheet: map_sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .build();
}

fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(MAP_WIDTH * 0.0, MAP_HEIGHT * 0.0, 1000.0);
    let mut cam = Camera::standard_3d(MAP_WIDTH, MAP_HEIGHT);
    {
        let proj = cam.projection_mut();
        let per = proj.as_perspective_mut().unwrap();
        println!("zMin: {}, zmax: {}", per.near(), per.far());
        per.set_far(10_000.0);
        println!("zMin: {}, zmax: {}", per.near(), per.far());
    }
    world
        .create_entity()
        .with(cam)
        .with(transform)
        .with(Zoomable {
            min_zoom: 10.0,
            max_zoom: 65536.0,
            zoom_rate: 20.0,
        })
        .with(Pannable {
            pan_rate: 20.0,
            min_x: -1.0 * (MAP_WIDTH / 2.0),
            min_y: -1.0 * (MAP_HEIGHT / 2.0),
            max_x: (MAP_WIDTH / 2.0),
            max_y: (MAP_HEIGHT / 2.0),
        })
        .with(AutoFov::new())
        .build();
}

/// Load a faction's badge based off of the provided name.
fn load_faction_badge(world: &mut World, faction_name: &str) -> SpriteRender {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        // load and return the texture handle
        loader.load(
            "texture/faction_badges/Bandits.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    let sprite_sheet_handle: Handle<SpriteSheet> = loader.load(
        "texture/faction_badges/badge.ron", // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    );

    return SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };
}

fn initialize_example_entities(world: &mut World) {
    let mut local_transform = Transform::default();
    let sprite_render = load_faction_badge(world, "bandits");
    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .build();
}
