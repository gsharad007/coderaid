use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use rand_core::RngCore;

#[derive(Debug)]
pub struct BotsPlugin;

impl Plugin for BotsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BotSpawnedEvent>()
            .init_resource::<BotSpawnerTimer>()
            .add_systems(Startup, bot_startup)
            .add_systems(Update, bot_spawning_system);
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
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

fn bot_startup() {
    // Setup your game world, camera, etc.
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::needless_pass_by_value)]
fn bot_spawning_system(
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
            0.,
            (rng.next_u32() % 16) as f32 - 5.,
        );
        let bot_entity = commands.spawn((Bot {}, transfrom)).id();

        bot_spawned_writer.send(BotSpawnedEvent {
            entity: bot_entity,
            transform: transfrom,
        });
    }
}
