use bevy::prelude::*;

use self::{
    resources::SelectedUnit,
    systems::{
        check_for_unit_movement, check_for_unit_selection, color_units, despawn_dead_units,
        test_spawn_unit,
    },
};

pub mod components;
pub mod resources;
mod systems;

const UNIT_LAYER: f32 = 2.0;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedUnit(None))
            .add_systems(PostStartup, test_spawn_unit)
            .add_systems(
                Update,
                (
                    despawn_dead_units,
                    check_for_unit_movement.before(check_for_unit_selection),
                    check_for_unit_selection,
                    color_units,
                ),
            );
    }
}
