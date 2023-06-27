use bevy::prelude::*;

#[derive(Component)]
pub struct Mobile;

#[derive(Component)]
pub struct StatBlock{
    hp: f32,
    armor: i32,
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Player;