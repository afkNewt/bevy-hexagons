use bevy::prelude::*;

use self::systems::place_enemy_capital;

mod systems;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, place_enemy_capital);
    }
}