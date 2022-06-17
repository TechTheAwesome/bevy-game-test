pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use systems::setup_paddle::setup_paddle;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
#[cfg(feature = "debug")]
#[derive(Inspectable, Component)]
struct InspectableType;
#[cfg(feature = "debug")]
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct ReflectedType;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_startup_system(setup_cameras)
        .add_startup_system(setup_paddle);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<InspectableType>()
        .register_type::<ReflectedType>();

    app.run();
}

fn setup_cameras(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
