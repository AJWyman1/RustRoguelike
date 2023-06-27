mod systems;
mod components;
mod events;

use events::*;
use components::*;
use systems::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .run();
}

