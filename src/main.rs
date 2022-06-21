pub mod components;
pub mod constants;
pub mod resources;
pub mod systems;

#[cfg(feature = "fps")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use components::CollisionEvent;
use constants::BACKGROUND_COLOR;
use resources::Scoreboard;
use systems::{
    button::{
        button_system::{button_system, start_button, remove_button},
    },
    setup_start_menu::setup_start_menu,
    setup_ball::setup_ball,
    setup_bricks::setup_bricks,
    setup_cameras::setup_cameras,
    setup_paddle::setup_paddle,
    setup_scoreboard::setup_scoreboard,
    setup_walls::setup_walls,
    system_collision::check_for_collisions,
    system_paddle::move_paddle,
    system_scoreboard::update_scoreboard,
    system_velocity::apply_velocity,
    system_win_condition::check_win_condition,
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    StartMenu,
    Playing,
    End,
}

fn main() {
    let mut app = App::new();
    // add state
    app.add_state(GameState::StartMenu);

    // add plugins
    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    });
    #[cfg(feature = "fps")]
    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    #[cfg(feature = "debug")] // world inspector has to go after default plugin to work
    app.add_plugin(WorldInspectorPlugin::new());

    app.insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR));

    app.add_event::<CollisionEvent>();

    // startup systems
    app.add_startup_system(setup_cameras);

    app.add_system_set(SystemSet::on_enter(GameState::StartMenu).with_system(setup_start_menu));
    app.add_system_set(
        SystemSet::on_enter(GameState::Playing)
            .with_system(setup_walls)
            .with_system(setup_scoreboard)
            .with_system(setup_bricks)
            .with_system(setup_ball)
            .with_system(setup_paddle)
            .with_system(remove_button),
    );

    // normal systems
    app.add_system(button_system)
        .add_system(bevy::input::system::exit_on_esc_system); //exit-on-escape :D

    app.add_system_set(SystemSet::on_update(GameState::StartMenu).with_system(start_button));

    app.add_system_set(
        SystemSet::on_update(GameState::Playing)
            .with_system(move_paddle)
            .with_system(apply_velocity)
            .with_system(update_scoreboard)
            .with_system(check_for_collisions)
            .with_system(check_win_condition),
    );

    // run app
    app.run();
}
