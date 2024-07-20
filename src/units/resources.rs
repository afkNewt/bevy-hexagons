use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedUnit(pub Option<Entity>);