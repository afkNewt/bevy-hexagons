use bevy::prelude::*;
use board::BoardPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

use text::TextPlugin;
use tile_highlighting::TileHighlighting;
use units::UnitsPlugin;

pub mod util;

pub mod board;
mod enemy;
mod player;
mod text;
mod tile_highlighting;
pub mod units;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb_u8(20, 20, 20)))
        .add_plugins(DefaultPlugins)
        .add_plugins((
            BoardPlugin,
            EnemyPlugin,
            PlayerPlugin,
            UnitsPlugin,
            TextPlugin,
            TileHighlighting,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
