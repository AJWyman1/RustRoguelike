use bevy::prelude::*; 

use crate::items::components::Weapon;

#[derive(Component)]
pub struct Player{
    pub equipped_weapon: Option<Weapon>,
}

#[derive(Component)]
pub struct PlayerData{
    player_entity: Entity,
    camera_entity: Entity,
}