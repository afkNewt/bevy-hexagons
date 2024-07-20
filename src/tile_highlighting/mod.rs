use bevy::prelude::*;

use self::systems::{
    color_tile_purpose_sprites, despawn_tile_purpose_sprites, highlight_hovered_hex,
    highlight_unit_hex, remove_tile_highlights, spawn_tile_purpose_sprites,
};

mod components;
mod systems;

const OVERLAY_LAYER: f32 = 3.0;

pub struct TileHighlighting;

impl Plugin for TileHighlighting {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                remove_tile_highlights,
                highlight_unit_hex,
                highlight_hovered_hex,
            )
                .chain(),
        )
        .add_systems(PreUpdate, despawn_tile_purpose_sprites)
        .add_systems(Update, spawn_tile_purpose_sprites)
        .add_systems(PostUpdate, color_tile_purpose_sprites);
    }
}
