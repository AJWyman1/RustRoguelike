use bevy::prelude::*;

#[derive(Component)]
pub struct Item{
    weight: i32,
    name: str,
}

#[derive(Component)]
pub struct Weapon{
    damage_type: DamageType,
    damage: i32,
}

pub enum DamageType{
    Frost,
    Fire,
    Slash,
    Blunt,
}

pub struct Potion;