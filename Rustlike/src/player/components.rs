use bevy::prelude::*; 


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerData{
    player_entity: Entity,
    camera_entity: Entity,
}