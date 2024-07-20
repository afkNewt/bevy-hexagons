use bevy::prelude::*;
use hexx::Hex;

use super::resources::HexColors;

#[derive(Component)]
pub struct Border;

#[derive(Reflect, PartialEq, Debug, Clone, Copy)]
pub enum Team {
    Neutral,
    Ally,
    Enemy,
}

#[derive(Reflect, PartialEq, Debug)]
pub enum TileVariant {
    Land,
    Capital,
}

#[derive(Component, PartialEq)]
pub struct HexTile {
    pub coordinate: Hex,
    pub variant: TileVariant,
    pub capture_progress: i32,
    pub team: Team,
}

impl HexTile {
    pub fn strong_highlight(&self, colors: &Res<HexColors>) -> Handle<ColorMaterial> {
        if self.team == Team::Ally && self.variant == TileVariant::Capital {
            return colors.ally_capital_strong_highlight.clone();
        }

        if self.team == Team::Enemy && self.variant == TileVariant::Capital {
            return colors.enemy_capital_strong_highlight.clone();
        }

        colors.neutral_strong_highlight.clone()
    }

    pub fn weak_highlight(&self, colors: &Res<HexColors>) -> Handle<ColorMaterial> {
        if self.team == Team::Ally && self.variant == TileVariant::Capital {
            return colors.ally_capital_weak_highlight.clone();
        }

        if self.team == Team::Enemy && self.variant == TileVariant::Capital {
            return colors.enemy_capital_weak_highlight.clone();
        }

        colors.neutral_weak_highlight.clone()
    }

    pub fn base_color(&self, colors: &Res<HexColors>) -> Handle<ColorMaterial> {
        if self.team == Team::Ally && self.variant == TileVariant::Capital {
            return colors.ally_capital.clone();
        }

        if self.team == Team::Enemy && self.variant == TileVariant::Capital {
            return colors.enemy_capital.clone();
        }

        colors.neutral.clone()
    }
}
