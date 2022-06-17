use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin, RegisterInspectable};
#[cfg(feature = "debug")]
#[derive(Inspectable, Component)]
struct InspectableType;
#[cfg(feature = "debug")]
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct ReflectedType;

fn main() {
    let mut app = App::new();
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<InspectableType>()
        .register_type::<ReflectedType>();

    app.add_plugins(DefaultPlugins);
    app.add_startup_system(setup_cameras);

    app.run();
}

fn setup_cameras(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
