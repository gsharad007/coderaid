mod camera_controller_plugin;
mod camera_setup_plugin;

mod game_scene_plugin;

use bevy::{app::App, DefaultPlugins};

use camera_controller_plugin::CameraControllerPlugin;
use camera_setup_plugin::CameraSetupPlugin;
use game_scene_plugin::SceneElementsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraSetupPlugin)
        .add_plugins(CameraControllerPlugin)
        .add_plugins(SceneElementsPlugin)
        // .insert_resource(WindowDescriptor {
        //     title: "CodeRaid".to_string(),
        //     ..default()
        // })
        .run();
}
