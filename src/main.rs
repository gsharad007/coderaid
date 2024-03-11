mod ibounds3;

mod game_bots_plugin;
mod game_cells_plugin;
mod game_coordinates_utils;
mod game_scene_plugin;
mod game_setup_data;

mod camera_controller_plugin;
mod camera_setup_plugin;

mod visual_bots_plugin;

use bevy::{
    app::App,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    DefaultPlugins,
};

use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;
use game_bots_plugin::BotsPlugin;

use camera_controller_plugin::CameraControllerPlugin;
use camera_setup_plugin::CameraSetupPlugin;
use game_scene_plugin::SceneElementsPlugin;
use game_setup_data::GameSetupData;
use visual_bots_plugin::VisualBotsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(GameSetupData)
        .add_plugins((CameraSetupPlugin, CameraControllerPlugin))
        .add_plugins(SceneElementsPlugin)
        .add_plugins(BotsPlugin)
        .add_plugins(VisualBotsPlugin)
        // .insert_resource(WindowDescriptor {
        //     title: "CodeRaid".to_string(),
        //     ..default()
        // })
        .run();
}
