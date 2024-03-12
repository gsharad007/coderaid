use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    ecs::system::Commands,
};

use crate::game_cells_plugin::Cells;
use crate::game_setup_data::MapData;
use crate::ibounds3::IBounds3;

#[derive(Debug)]
pub struct SceneElementsPlugin;

impl Plugin for SceneElementsPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_event::<CellsSpawnedEvent>()
            .add_systems(Startup, spawn_scene_cells);
    }
}

#[derive(Event, Debug)]
// Define an event to represent the spawning of a bot
pub struct CellsSpawnedEvent {
    // pub cells: Cells,
    // pub map_data: MapData,
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_precision_loss)]
fn spawn_scene_cells(
    mut commands: Commands,
    mut cells_spawned_writer: EventWriter<CellsSpawnedEvent>,
) {
    #[allow(clippy::non_ascii_literal)]
    let cells_string = "
█╞╦╗╔╦╩╡╚══════╗
╞═╬╣╠╬╦╡╔══════╝
██║║╠╣║█║╔═════╗
╞═╩╝╠╣║█║║╔════╝
╔╗╔╗╚╝║█║║║╔╦╦╦╗
╝╚╝╚╗╔╝█║║╚╩╩╩╩╝
████╚╝██║╚═════╗
████╔╗██╚══════╝
";

    let cells = Cells::from_string(cells_string);

    let map_data = MapData {
        bounds: IBounds3::new(IVec3::ZERO, cells.size),
    };

    commands.insert_resource(cells);
    commands.insert_resource(map_data);

    _ = cells_spawned_writer.send(CellsSpawnedEvent {});
}
