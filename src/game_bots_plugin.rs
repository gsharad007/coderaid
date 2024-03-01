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

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::needless_pass_by_value)]
fn bots_spawning_system(
    time: Res<Time>,
    mut commands: Commands,
    mut bot_spawner_timer: ResMut<BotSpawnerTimer>,
    mut bot_spawned_writer: EventWriter<BotSpawnedEvent>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // Update the timer with the time elapsed since the last update
    if bot_spawner_timer.0.tick(time.delta()).just_finished() {
        // Timer has finished, so spawn a new bot

        let transfrom = Transform::from_xyz(
            (rng.next_u32() % 16) as f32 - 5.,
            0.1,
            (rng.next_u32() % 16) as f32 - 5.,
        );
        let bot_entity = commands.spawn((Bot {}, transfrom)).id();

        bot_spawned_writer.send(BotSpawnedEvent {
            entity: bot_entity,
            transform: transfrom,
        });
    }
}

#[allow(clippy::needless_pass_by_value)]
fn bots_movement_system(time: Res<Time>, mut query: Query<&mut Transform, With<Bot>>) {
    for mut transform in query.iter_mut() {
        let move_delta = transform.forward() * BOT_MOVEMENT_SPEED * time.delta_seconds();
        transform.translation += move_delta;
    }
}
