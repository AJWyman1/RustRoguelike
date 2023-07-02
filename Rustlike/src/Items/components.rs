use bevy::prelude::*;

#[derive(Component)]
pub struct Item{
    weight: f32,
    name: String,
    durability: f32,
}

#[derive(Component)]
pub struct Weapon{
    damage_type: DamageType,
    damage: f32,
    melee: bool,
    range: f32,
}

pub enum DamageType{
    Frost,
    Fire,
    Slash,
    Blunt,
}

#[derive(Component)]
pub struct Potion;

#[derive(Component)]
pub struct Armor{
    armor_type: ArmorType,
    armor_class: f32,
    dex_mod: bool,
    metal: bool,
    stealth_disadvantage: bool,
}

pub enum ArmorType{
    Light,
    Medium,
    Heavy,
    Shield,
}