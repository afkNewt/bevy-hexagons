use bevy::prelude::*;

use crate::{
    board::components::{HexTile, Team, TileVariant},
    units::components::Unit,
    util::cursor_to_hex,
};

use super::resources::{AllyCapital, PlayerCoins, TurnCounter};

pub fn place_ally_capital(
    mut ally_capital: ResMut<AllyCapital>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut hexes: Query<&mut HexTile>,
) {
    if ally_capital.position.is_some() {
        return;
    }
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(hovered_hex) = cursor_to_hex(windows) else {
        return;
    };

    let claim_tiles = hovered_hex.all_neighbors();

    for hex in &hexes {
        if !claim_tiles.contains(&hex.coordinate) {
            continue;
        }

        if hex.team == Team::Enemy {
            return;
        }
    }

    for mut hex in &mut hexes {
        if claim_tiles.contains(&hex.coordinate) {
            hex.team = Team::Ally;
        }

        if hex.coordinate == hovered_hex {
            hex.team = Team::Ally;
            hex.variant = TileVariant::Capital;
            ally_capital.position = Some(hovered_hex);
        }
    }
}

pub fn pass_turn(
    mut turn_counter: ResMut<TurnCounter>,
    mut player_coin_count: ResMut<PlayerCoins>,
    mut units: Query<&mut Unit>,
    hex_tiles: Query<&mut HexTile>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if !keys.just_released(KeyCode::Space) {
        return;
    }

    for hex_tile in &hex_tiles {
        if hex_tile.team == Team::Ally {
            player_coin_count.0 += 1;
        }
    }

    for mut unit in &mut units {
        unit.new_turn();
    }

    // give the player a garanteed
    // 2 coins per turn
    player_coin_count.0 += 2;

    turn_counter.0 += 1;

    update_capture_progress(hex_tiles, units.to_readonly());
}

fn update_capture_progress(mut tiles: Query<&mut HexTile>, units: Query<&Unit>) {
    let progress_capture_tiles = tiles
        .iter()
        .filter_map(|hex_tile| {
            let Some(unit) = units.iter().find(|u| u.position == hex_tile.coordinate) else {
                return None;
            };

            if unit.team == hex_tile.team {
                return None;
            }

            let neighbors = hex_tile.coordinate.all_neighbors();
            if tiles.iter().any(|neighbor| {
                neighbors.contains(&neighbor.coordinate) && unit.team == neighbor.team
            }) {
                return Some((hex_tile.coordinate, unit.team));
            }

            None
        })
        .collect::<Vec<_>>();

    for mut tile in &mut tiles {
        let mut capture_team = None;
        for (coord, team) in &progress_capture_tiles {
            if tile.coordinate != *coord {
                continue;
            }

            capture_team = Some(*team);
            break;
        }

        match (capture_team, tile.team) {
            (None, Team::Neutral) => {
                move_toward(&mut tile.capture_progress, 0, 1);
            }
            (None, _) => {
                move_toward(&mut tile.capture_progress, 3, 1);
            }
            (Some(team), Team::Neutral) => {
                if move_toward(&mut tile.capture_progress, 3, 1) {
                    tile.team = team
                }
            }
            (Some(_), _) => {
                if move_toward(&mut tile.capture_progress, 0, 1) {
                    tile.team = Team::Neutral
                }
            }
        };
    }
}

fn move_toward(value: &mut i32, target: i32, step: i32) -> bool {
    *value = (*value - step).max((*value + step).min(target));
    *value == target
}
