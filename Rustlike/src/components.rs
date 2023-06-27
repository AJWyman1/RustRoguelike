use bevy::prelude::*;

#[derive(Component)]
pub struct Card{
    pub value: u8,
    pub suit: Suit,
}

pub enum Suit{
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Component)]
pub struct Deck;