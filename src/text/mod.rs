use bevy::prelude::*;

use self::systems::{
    generate_player_coin_text, generate_tile_info_text, update_player_coin_text,
    update_tile_info_text,
};

mod components;
pub mod systems;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (generate_tile_info_text, generate_player_coin_text))
            .add_systems(Update, (update_tile_info_text, update_player_coin_text));
    }
}
