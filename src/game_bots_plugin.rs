use core::time::Duration;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

use crate::game_coordinates_utils::CellCoords;
use crate::game_setup_data::MapData;
use crate::ibounds3::IBounds3;

const BOT_SPAWNING_INTERVAL: f32 = 2.;
const BOT_MOVEMENT_SPEED: f32 = 1.;

#[derive(Debug)]
pub struct BotsPlugin;

impl Plugin for BotsPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_event::<BotSpawnedEvent>()
            .init_resource::<BotSpawnerTimer>()
            // .add_systems(Startup, bots_startup)
            .add_systems(Update, (bots_spawning_system, bots_movement_system));
    }
}

#[derive(Component, Debug)]
pub struct Bot {}

#[derive(Event, Debug)]
// Define an event to represent the spawning of a bot
pub struct BotSpawnedEvent {
    pub entity: Entity,
    pub transform: Transform,
}

#[derive(Resource, Debug)]
struct BotSpawnerTimer(Timer);

impl Default for BotSpawnerTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            BOT_SPAWNING_INTERVAL,
            TimerMode::Repeating,
        ))
    }
}

// fn bots_startup() {
//     // Setup your game world, camera, etc.
// }

#[allow(clippy::needless_pass_by_value)]
fn bots_spawning_system(
    time: Res<Time>,
    commands: Commands,
    map_data: Res<MapData>,
    bot_spawner_timer: ResMut<BotSpawnerTimer>,
    bot_spawned_writer: EventWriter<BotSpawnedEvent>,
    rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // Update the timer with the time elapsed since the last update
    if bot_spawner_timer_just_finishes(time.delta(), bot_spawner_timer) {
        // Timer has finished, so spawn a new bot
        spawn_bot_on_map_trigger_event(commands, &map_data.bounds, bot_spawned_writer, rng);
    }
}

fn bot_spawner_timer_just_finishes(
    time_delta: Duration,
    mut bot_spawner_timer: ResMut<BotSpawnerTimer>,
) -> bool {
    bot_spawner_timer.0.tick(time_delta).just_finished()
}

fn spawn_bot_on_map_trigger_event(
    commands: Commands,
    map_bounds: &IBounds3,
    mut bot_spawned_writer: EventWriter<BotSpawnedEvent>,
    rng: ResMut<GlobalEntropy<WyRand>>,
) {
    let (transfrom, bot_entity) = spawn_bot_on_map(commands, map_bounds, rng);

    _ = bot_spawned_writer.send(BotSpawnedEvent {
        entity: bot_entity,
        transform: transfrom,
    });
}

fn spawn_bot_on_map(
    commands: Commands,
    map_bounds: &IBounds3,
    rng: ResMut<GlobalEntropy<WyRand>>,
) -> (Transform, Entity) {
    let cell_indices = generate_random_cell_indices(rng, map_bounds);
    let transfrom = cell_indices.as_game_coordinates_transform();
    let bot_entity = spawn_bot_with_transform(commands, transfrom);
    (transfrom, bot_entity)
}

#[allow(clippy::cast_possible_wrap)]
fn generate_random_cell_indices(
    mut rng: ResMut<'_, GlobalEntropy<WyRand>>,
    map_bounds: &IBounds3,
) -> CellCoords {
    let size = map_bounds.size().as_uvec3();
    let random_coords = UVec3::new(rng.next_u32(), rng.next_u32(), rng.next_u32());
    let random_coords_limited = random_coords % size;
    CellCoords::from_ivec3(random_coords_limited.as_ivec3() + map_bounds.min)
}

fn spawn_bot_with_transform(mut commands: Commands, transfrom: Transform) -> Entity {
    let bot_entity = commands
        .spawn((Bot {}, TransformBundle::from(transfrom)))
        .id();
    bot_entity
}

#[allow(clippy::needless_pass_by_value)]
fn bots_movement_system(time: Res<Time>, mut query: Query<&mut Transform, With<Bot>>) {
    for mut transform in &mut query {
        let move_delta = transform.forward() * BOT_MOVEMENT_SPEED * time.delta_seconds();
        transform.translation += move_delta;
    }
}
