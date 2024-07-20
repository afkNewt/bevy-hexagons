use bevy::prelude::*;

#[derive(Resource)]
pub struct HexColors {
    pub backround_hex: Handle<ColorMaterial>,

    pub neutral: Handle<ColorMaterial>,
    pub neutral_weak_highlight: Handle<ColorMaterial>,
    pub neutral_strong_highlight: Handle<ColorMaterial>,

    pub ally_sprite: Color,
    pub ally_unused_action_color: Color,
    pub ally_used_action_color: Color,
    pub ally_border_color: Handle<ColorMaterial>,
    pub ally_capital: Handle<ColorMaterial>,
    pub ally_capital_weak_highlight: Handle<ColorMaterial>,
    pub ally_capital_strong_highlight: Handle<ColorMaterial>,

    pub enemy_sprite: Color,
    pub enemy_unused_action_color: Color,
    pub enemy_used_action_color: Color,
    pub enemy_border_color: Handle<ColorMaterial>,
    pub enemy_capital: Handle<ColorMaterial>,
    pub enemy_capital_weak_highlight: Handle<ColorMaterial>,
    pub enemy_capital_strong_highlight: Handle<ColorMaterial>,
}
