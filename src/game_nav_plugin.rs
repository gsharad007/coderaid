
use bevy::prelude::*;

use crate::{game_cells_plugin::Cells, game_scene_plugin::CellsSpawnedEvent, game_setup_data::MapData};

#[derive(Debug)]
pub struct NavPlugin;

impl Plugin for NavPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_systems(Update, create_scene)
    }
}

/// Creates the scene elements (floor, walls, ceiling)
fn create_scene(
    mut commands: Commands,
    mut cells_spawned_reader: EventReader<CellsSpawnedEvent>,
    cells: Option<Res<Cells>>,
    map_data: Option<Res<MapData>>,
) {
    if let Some(cells) = cells {
        if let Some(map_data) = map_data {
            for CellsSpawnedEvent {} in cells_spawned_reader.read() {
            }
        }
    }
}
