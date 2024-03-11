use bevy::prelude::*;

use crate::ibounds3::IBounds3;

#[derive(Debug)]
pub struct GameSetupData;

impl Plugin for GameSetupData {
    fn build(&self, app: &mut App) {
        _ = app.init_resource::<MapData>();
    }
}

#[derive(Resource, Debug, Default)]
pub struct MapData {
    pub bounds: IBounds3,
}
