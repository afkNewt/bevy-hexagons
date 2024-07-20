use bevy::prelude::*;

use crate::units::components::Action;

#[derive(Component)]
pub struct TilePurposeSprite(pub Action);
