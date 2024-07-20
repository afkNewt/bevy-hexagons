use bevy::prelude::*;
use hexx::Hex;

use crate::board::{HEX_LAYOUT, HEX_RADIUS};

pub fn cursor_to_hex(windows: Query<&Window>) -> Option<Hex> {
    let Ok(primary) = windows.get_single() else {
        return None;
    };

    let Some(cursor_pos) = primary.cursor_position() else {
        return None;
    };

    let x = cursor_pos.x - primary.resolution.width() / 2.;
    let y = (cursor_pos.y - primary.resolution.height() / 2.) * -1.;

    let cursors_hex_pos = HEX_LAYOUT.world_pos_to_hex(hexx::Vec2::new(x, y));

    if Hex::ZERO
        .range(HEX_RADIUS as u32)
        .collect::<Vec<Hex>>()
        .contains(&cursors_hex_pos)
    {
        Some(cursors_hex_pos)
    } else {
        None
    }
}
