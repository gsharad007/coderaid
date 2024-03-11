use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::{PbrBundle, StandardMaterial},
    render::{color::Color, mesh::Mesh},
};

use crate::game_bots_plugin::BotSpawnedEvent;

#[derive(Debug)]
pub struct VisualBotsPlugin;

impl Plugin for VisualBotsPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            // .add_systems(Startup, startup_visual_bots)
            .add_systems(Update, on_bot_spawned_listener_system);
    }
}

// /// TODO: Preinitialize assets
// fn startup_visual_bots() {}

fn on_bot_spawned_listener_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bot_spawned_reader: EventReader<BotSpawnedEvent>,
) {
    for event in bot_spawned_reader.read() {
        if let Some(mut entity_command) = commands.get_entity(event.entity) {
            _ = entity_command.with_children(|parent| {
                _ = parent.spawn(PbrBundle {
                    mesh: meshes.add(Cylinder::new(0.4, 0.1)),
                    material: materials.add(Color::rgb(0.6, 0.7, 0.9)),
                    transform: Transform::from_xyz(0.5, 0.5, 0.5),
                    ..default()
                });
            });
        }
    }
}
