use bevy::prelude::*;

use crate::ibounds3::IBounds3;

#[derive(Debug)]
pub struct GameSetupData;

impl Plugin for GameSetupData {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Resource, Copy, Clone, Debug, Default)]
pub struct MapData {
    pub bounds: IBounds3,
}
