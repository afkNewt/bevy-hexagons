use bevy::prelude::*;
use hexx::Hex;

use crate::{
    board::{
        components::{HexTile, Team},
        resources::HexColors,
        HEX_LAYOUT, HEX_RADIUS, HEX_SIZE,
    },
    units::{
        components::{Action, Unit},
        resources::SelectedUnit,
    },
    util::cursor_to_hex,
};

use super::{components::TilePurposeSprite, OVERLAY_LAYER};

pub fn remove_tile_highlights(
    mut hexes: Query<(&HexTile, &mut Handle<ColorMaterial>)>,
    colors: Res<HexColors>,
) {
    for (hex, mut color_mat) in &mut hexes {
        *color_mat = hex.base_color(&colors);
    }
}

pub fn highlight_hovered_hex(
    windows: Query<&Window>,
    mut hexes: Query<(&HexTile, &mut Handle<ColorMaterial>)>,
    colors: Res<HexColors>,
) {
    let Some(hovered_hex) = cursor_to_hex(windows) else {
        return;
    };

    let hovered = hexes.iter_mut().find(|(h, _)| h.coordinate == hovered_hex);

    if let Some((hex, mut color_mat)) = hovered {
        *color_mat = hex.strong_highlight(&colors);
    }
}

pub fn highlight_unit_hex(
    selected_unit: Res<SelectedUnit>,
    units: Query<&Unit>,
    mut hexes: Query<(&HexTile, &mut Handle<ColorMaterial>)>,
    colors: Res<HexColors>,
) {
    let Some(selected_entity) = selected_unit.0 else {
        return;
    };

    let Ok(unit) = units.get(selected_entity) else {
        return;
    };

    let mut strong_highlights = Vec::new();
    let mut weak_highlights = Vec::new();

    if unit.actions.contains(&Action::Attack) {
        strong_highlights.append(&mut unit.relative_attack_hexes());
    } else {
        weak_highlights.append(&mut unit.relative_attack_hexes());
    };

    if unit.actions.contains(&Action::Move) {
        strong_highlights.append(&mut unit.relative_move_hexes());
    } else {
        weak_highlights.append(&mut unit.relative_move_hexes());
    };

    for (hex, mut color_mat) in &mut hexes {
        if strong_highlights.contains(&hex.coordinate) {
            *color_mat = hex.strong_highlight(&colors);
            continue;
        }

        if weak_highlights.contains(&hex.coordinate) {
            *color_mat = hex.weak_highlight(&colors);
            continue;
        }
    }
}

pub fn spawn_tile_purpose_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected_unit: Res<SelectedUnit>,
    units: Query<&Unit>,
) {
    let Some(selected_entity) = selected_unit.0 else {
        return;
    };

    let Ok(unit) = units.get(selected_entity) else {
        return;
    };

    let valid_hex_tiles = Hex::ZERO.range(HEX_RADIUS as u32).collect::<Vec<Hex>>();

    let mut both = unit.relative_move_hexes();
    both.retain(|cube| unit.relative_attack_hexes().contains(cube));

    for hex in unit.relative_move_hexes() {
        if !valid_hex_tiles.contains(&hex) {
            continue;
        }

        let pixel_position = HEX_LAYOUT.hex_to_world_pos(hex);

        let transform = if both.contains(&hex) {
            Transform {
                translation: Vec3::new(
                    pixel_position.x + HEX_SIZE / 3.,
                    pixel_position.y,
                    OVERLAY_LAYER,
                ),
                scale: Vec3::splat(HEX_SIZE / 220.),
                ..Default::default()
            }
        } else {
            Transform {
                translation: Vec3::from_array(pixel_position.extend(OVERLAY_LAYER).to_array()),
                scale: Vec3::splat(HEX_SIZE / 220.),
                ..Default::default()
            }
        };

        commands
            .spawn(SpriteBundle {
                transform,
                texture: asset_server.load("sprites/move.png".to_string()),
                ..default()
            })
            .insert(TilePurposeSprite(Action::Move));
    }

    for hex in unit.relative_attack_hexes() {
        if !valid_hex_tiles.contains(&hex) {
            continue;
        }

        let pixel_position = HEX_LAYOUT.hex_to_world_pos(hex);

        let transform = if both.contains(&hex) {
            Transform {
                translation: Vec3::new(
                    pixel_position.x - HEX_SIZE / 3.,
                    pixel_position.y,
                    OVERLAY_LAYER,
                ),
                scale: Vec3::splat(HEX_SIZE / 220.),
                ..Default::default()
            }
        } else {
            Transform {
                translation: Vec3::from_array(pixel_position.extend(OVERLAY_LAYER).to_array()),
                scale: Vec3::splat(HEX_SIZE / 220.),
                ..Default::default()
            }
        };

        commands
            .spawn(SpriteBundle {
                transform,
                texture: asset_server.load("sprites/attack.png".to_string()),
                ..default()
            })
            .insert(TilePurposeSprite(Action::Attack));
    }
}

pub fn despawn_tile_purpose_sprites(
    mut commands: Commands,
    tile_purpose_sprites: Query<Entity, With<TilePurposeSprite>>,
) {
    for entity in &tile_purpose_sprites {
        commands.entity(entity).despawn();
    }
}

pub fn color_tile_purpose_sprites(
    mut sprites: Query<(&TilePurposeSprite, &mut Sprite)>,
    colors: Res<HexColors>,
    selected_unit: Res<SelectedUnit>,
    units: Query<&Unit>,
) {
    let Some(selected_entity) = selected_unit.0 else {
        return;
    };

    let Ok(unit) = units.get(selected_entity) else {
        return;
    };

    let unused_color = match unit.team {
        Team::Ally => colors.ally_unused_action_color,
        Team::Enemy => colors.enemy_unused_action_color,
        _ => colors.ally_unused_action_color,
    };

    let used_color = match unit.team {
        Team::Ally => colors.ally_used_action_color,
        Team::Enemy => colors.enemy_used_action_color,
        _ => colors.ally_used_action_color,
    };

    for (tile_purpose_sprite, mut sprite) in &mut sprites {
        if unit.actions.contains(&tile_purpose_sprite.0) {
            sprite.color = unused_color;
        } else {
            sprite.color = used_color;
        }
    }
}
