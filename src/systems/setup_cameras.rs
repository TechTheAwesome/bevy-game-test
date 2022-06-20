use bevy::prelude::*;

use crate::components::MainCamera;


pub fn setup_cameras(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
}