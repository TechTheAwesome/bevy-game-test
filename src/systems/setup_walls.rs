use bevy::prelude::Commands;
use crate::components::wall_bundle::{WallBundle, WallLocation};

pub fn setup_walls(mut commands: Commands) {
    // Walls
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}