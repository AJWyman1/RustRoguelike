use bevy::prelude::*;

#[derive(Component)]
pub struct Mobile;

#[derive(Component)]
pub struct StatBlock{
    pub hp: f32,
    pub armor: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

#[derive(Component)]
pub struct Enemy;


#[derive(Component)]
pub struct Crosshair;