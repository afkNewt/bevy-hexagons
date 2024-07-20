use bevy::prelude::*;
use hexx::HexLayout;

use self::systems::{build_board, draw_borders, load_colors};

pub mod components;
pub mod resources;
pub mod systems;

// how many hex_rads larger the
// background hex should be
pub const BACKGROUND_HEX_SIZE: f32 = 1.8;

pub const HEX_SIZE: f32 = 40.;
pub const HEX_GAP: f32 = 2.5;
pub const HEX_RADIUS: i32 = 5;

pub const HEX_LAYOUT: HexLayout = HexLayout {
    orientation: hexx::HexOrientation::Pointy,
    origin: hexx::Vec2::ZERO,
    hex_size: hexx::Vec2::splat(HEX_SIZE + HEX_GAP),
    invert_x: false,
    invert_y: true,
};

const BACKGROUND_HEX_LAYER: f32 = 0.0;
const HEX_LAYER: f32 = 1.0;
const BORDER_LAYER: f32 = 2.0;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_colors)
            .add_systems(Startup, build_board)
            .add_systems(Update, draw_borders);
    }
}
