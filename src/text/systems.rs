use bevy::prelude::*;

use crate::{board::components::HexTile, player::resources::PlayerCoins, util::cursor_to_hex};

use super::components::{CoinText, TileText};

pub fn generate_tile_info_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shiny_color = Color::srgb(
        bevy::color::palettes::css::GOLD.red,
        bevy::color::palettes::css::GOLD.green,
        bevy::color::palettes::css::GOLD.blue,
    );
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Tile: ",
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/arial.ttf"),
                font_size: 40.0,
                color: shiny_color,
            }),
            TextSection::new(
                "\nCapture: ",
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/arial.ttf"),
                font_size: 40.0,
                color: shiny_color,
            }),
            TextSection::new(
                "\nTeam: ",
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/arial.ttf"),
                font_size: 40.0,
                color: shiny_color,
            }),
        ]),
        TileText,
    ));
}

pub fn update_tile_info_text(
    windows: Query<&Window>,
    hexes: Query<&HexTile>,
    mut tile_text: Query<&mut Text, With<TileText>>,
) {
    let Some(hovered_hex) = cursor_to_hex(windows) else {
        return;
    };

    let mut hex_coord = "None".to_string();
    let mut hex_capture = "??".to_string();
    let mut hex_team = "??".to_string();

    for hex in &hexes {
        if hex.coordinate == hovered_hex {
            hex_coord = format!("{:?} {:?}", hex.variant, hex.coordinate);
            hex_capture = format!("{}", hex.capture_progress);
            hex_team = format!("{:?}", hex.team);
        }
    }

    for mut text in &mut tile_text {
        // Update the value of the second section
        text.sections[1].value = hex_coord.to_string();
        text.sections[3].value = hex_capture.to_string();
        text.sections[5].value = hex_team.to_string();
    }
}

pub fn generate_player_coin_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shiny_color = Color::srgb(
        bevy::color::palettes::css::GOLD.red,
        bevy::color::palettes::css::GOLD.green,
        bevy::color::palettes::css::GOLD.blue,
    );
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "\n\n\nCoins: ",
                TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/arial.ttf"),
                font_size: 40.0,
                color: shiny_color,
            }),
        ]),
        CoinText,
    ));
}

pub fn update_player_coin_text(
    player_coins: Res<PlayerCoins>,
    mut coin_text: Query<&mut Text, With<CoinText>>,
) {
    for mut text in &mut coin_text {
        // Update the value of the second section
        text.sections[1].value = format!("{}", player_coins.0);
    }
}
