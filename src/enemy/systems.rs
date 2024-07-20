use bevy::prelude::*;
use hexx::hex;

use crate::board::components::{HexTile, TileVariant, Team};

pub fn place_enemy_capital(mut hexes: Query<&mut HexTile>) {
    let enemy_capital = hex(0, 0);
    let enemy_tiles = enemy_capital.all_neighbors();

    for mut hex in &mut hexes {
        if enemy_tiles.contains(&hex.coordinate) {
            hex.team = Team::Enemy;
        }

        if hex.coordinate == enemy_capital {
            hex.team = Team::Enemy;
            hex.variant = TileVariant::Capital;
        }
    }
}
