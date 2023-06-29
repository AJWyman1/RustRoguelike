use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::*;
use crate::components::{Mobile, StatBlock};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    commands.spawn(
        (
            Player,
            Mobile,
            StatBlock { hp: 10.0, armor: 10, strength: 10, dexterity: 10, constitution: 10, intelligence: 10, wisdom: 10, charisma: 10 },
            SpriteBundle{
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0).with_scale(Vec3 { x: 0.10, y: 0.1, z: 0.0 }),
                texture: asset_server.load("textures/player.png"),
                ..default()
            }
        )
    );
}

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
){
    let mut player_transform = player_query.get_single_mut().unwrap(); //Might have to change if multi-player is added
    let movespeed = 5.0;

    if keyboard_input.pressed(KeyCode::W){player_transform.translation.y += movespeed;}
    if keyboard_input.pressed(KeyCode::A){player_transform.translation.x -= movespeed;}
    if keyboard_input.pressed(KeyCode::S){player_transform.translation.y -= movespeed;}
    if keyboard_input.pressed(KeyCode::D){player_transform.translation.x += movespeed;}
}