use bevy::prelude::*;
use hexx::Hex;

#[derive(Resource)]
pub struct AllyCapital {
    pub position: Option<Hex>,
}

#[derive(Resource)]
pub struct TurnCounter(pub i32);

#[derive(Resource)]
pub struct PlayerCoins(pub i32);
