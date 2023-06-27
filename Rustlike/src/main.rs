mod systems;
mod components;
mod events;

use events::*;
use components::*;
use systems::*;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            Player,
            Mobile,
            StatBlock { hp: 10.0, armor: 10, strength: 10, dexterity: 10, constitution: 10, intelligence: 10, wisdom: 10, charisma: 10 },
            SpriteBundle{
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: 
                ..default()
            }
        )
    );
}