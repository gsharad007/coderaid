use std::time::Duration;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

const BOT_SPAWNING_INTERVAL: f32 = 2.;
const BOT_MOVEMENT_SPEED: f32 = 1.;

#[derive(Debug)]
pub struct BotsPlugin;

impl Plugin for BotsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BotSpawnedEvent>()
            .init_resource::<BotSpawnerTimer>()
            .add_systems(Startup, bots_startup)
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

fn bots_startup() {
    // Setup your game world, camera, etc.
}

#[allow(clippy::needless_pass_by_value)]
fn bots_spawning_system(
    time: Res<Time>,
    commands: Commands,
    bot_spawner_timer: ResMut<BotSpawnerTimer>,
    bot_spawned_writer: EventWriter<BotSpawnedEvent>,
    rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // Update the timer with the time elapsed since the last update
    if bot_spawner_timer_just_finishes(time.delta(), bot_spawner_timer) {
        // Timer has finished, so spawn a new bot
        spawn_bot_on_map_trigger_event(rng, commands, bot_spawned_writer);
    }
}

fn bot_spawner_timer_just_finishes(
    time_delta: Duration,
    mut bot_spawner_timer: ResMut<BotSpawnerTimer>,
) -> bool {
    bot_spawner_timer.0.tick(time_delta).just_finished()
}

fn spawn_bot_on_map_trigger_event(
    rng: ResMut<GlobalEntropy<WyRand>>,
    commands: Commands,
    mut bot_spawned_writer: EventWriter<BotSpawnedEvent>,
) {
    let (transfrom, bot_entity) = spawn_bot_on_map(commands, rng);

    bot_spawned_writer.send(BotSpawnedEvent {
        entity: bot_entity,
        transform: transfrom,
    });
}

#[allow(clippy::cast_precision_loss)]
fn spawn_bot_on_map(
    commands: Commands,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) -> (Transform, Entity) {
    let transfrom = Transform::from_xyz(
        (rng.next_u32() % 16) as f32 - 5.,
        0.0,
        (rng.next_u32() % 16) as f32 - 5.,
    );
    let bot_entity = spawn_bot_with_transform(commands, transfrom);
    (transfrom, bot_entity)
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
