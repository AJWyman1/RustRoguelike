mod systems;
mod components;
mod events;
mod creatures;
mod player;

use crate::player::PlayerPlugin;

use events::*;
use components::*;
use systems::*;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_crosshair)
        .add_system(move_crosshair)
        .run();
}

fn spawn_crosshair(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle{
            transform: Transform::from_xyz(window.width() / 2.0 + 90.0, window.height() / 2.0, 0.0).with_scale(Vec3{x: 0.05, y: 0.05, z: 0.05}),
            texture: asset_server.load("textures/crosshair.png"),
            ..default()
        },
        Crosshair,
    ));
}

fn move_crosshair(
    mut crosshair_query: Query<&mut Transform, With<Crosshair>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    if let Some(cursor) = window_query.single().cursor_position() {
        crosshair_query.get_single_mut().unwrap().translation.x = cursor.x;
        crosshair_query.get_single_mut().unwrap().translation.y = cursor.y;
    }
}