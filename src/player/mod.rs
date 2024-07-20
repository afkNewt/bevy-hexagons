use bevy::prelude::*;

use self::{
    resources::{AllyCapital, PlayerCoins, TurnCounter},
    systems::{pass_turn, place_ally_capital}
};

pub mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AllyCapital { position: None })
            .insert_resource(TurnCounter(0))
            .insert_resource(PlayerCoins(10))
            .add_systems(Update, (place_ally_capital, pass_turn));
    }
}
