pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use systems::{
    setup_ball::setup_ball, setup_bricks::setup_bricks, setup_cameras::setup_cameras,
    setup_paddle::setup_paddle, setup_scoreboard::setup_scoreboard, setup_walls::setup_walls,
    system_collision::check_for_collisions, system_paddle::move_paddle,
    system_velocity::apply_velocity,
};

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
    // add plugins
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")] // world inspector has to go after default plugin to work
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<InspectableType>()
        .register_type::<ReflectedType>();

    // startup systems
    app.add_startup_system(setup_cameras)
        .add_startup_system(setup_paddle)
        .add_startup_system(setup_walls)
        .add_startup_system(setup_ball)
        .add_startup_system(setup_scoreboard)
        .add_startup_system(setup_bricks);

    // normal systems
    app.add_system(move_paddle)
        .add_system(apply_velocity)
        .add_system(check_for_collisions);

    // run app
    app.run();
}
