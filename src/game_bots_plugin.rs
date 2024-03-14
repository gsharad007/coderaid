use core::f32::consts::FRAC_PI_2;
use core::time::Duration;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use bevy_xpbd_3d::components::{CollisionLayers, ExternalForce, RigidBody};
use bevy_xpbd_3d::plugins::collision::Collider;
use bevy_xpbd_3d::prelude::*;
use rand_core::RngCore;

use crate::game_cells_plugin::{cell, Cells};
use crate::game_coordinates_utils::CellCoords;
use crate::game_physics_layers::Layer;
use crate::game_setup_data::MapData;
use crate::ibounds3::IBounds3;

const BOT_SPAWNING_INTERVAL: f32 = 0.5;
const BOT_LOGIC_UPDATE_INTERVAL: f32 = 0.25;

const BOT_MOVEMENT_SPEED: f32 = 1.;
const BOT_MASS_DENSITY_SCALE: f32 = 0.25;

#[derive(Debug)]
pub struct BotsPlugin;

impl Plugin for BotsPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_event::<BotSpawnedEvent>()
            .init_resource::<BotSpawnerTimer>()
            .init_resource::<BotLogicUpdateTimer>()
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
        Self(Timer::from_seconds(BOT_SPAWNING_INTERVAL, TimerMode::Once))
    }
}

#[derive(Resource, Debug)]
struct BotLogicUpdateTimer(Timer);

impl Default for BotLogicUpdateTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            BOT_LOGIC_UPDATE_INTERVAL,
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
    mut bot_spawner_timer: ResMut<BotSpawnerTimer>,
    bot_spawned_writer: EventWriter<BotSpawnedEvent>,
    rng: ResMut<GlobalEntropy<WyRand>>,
) {
    // Update the timer with the time elapsed since the last update
    if timer_just_finishes(time.delta(), &mut bot_spawner_timer.0) {
        // Timer has finished, so spawn a new bot
        spawn_bot_on_map_trigger_event(commands, &map_data.bounds, bot_spawned_writer, rng);
    }
}

fn timer_just_finishes(time_delta: Duration, timer: &mut Timer) -> bool {
    timer.tick(time_delta).just_finished()
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
    let cell_coords = generate_random_cell_coords(rng, map_bounds);
    let transfrom = cell_coords.as_game_coordinates_transform();
    let bot_entity = spawn_bot_with_transform(commands, transfrom);
    (transfrom, bot_entity)
}

#[allow(clippy::cast_possible_wrap)]
fn generate_random_cell_coords(
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    map_bounds: &IBounds3,
) -> CellCoords {
    let size = map_bounds.size().as_uvec3();
    let random_coords = (-map_bounds.min).as_uvec3();
    // let random_coords = UVec3::new(rng.next_u32(), rng.next_u32(), rng.next_u32());
    let random_coords_limited = random_coords % size;
    CellCoords::from_ivec3(random_coords_limited.as_ivec3() + map_bounds.min)
}

fn spawn_bot_with_transform(mut commands: Commands, transform: Transform) -> Entity {
    let collider = Collider::cylinder(0.1, 0.25);
    let bot_entity = commands
        .spawn((
            Bot {},
            SpatialBundle::from_transform(transform),
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&collider, BOT_MASS_DENSITY_SCALE),
            collider,
            CollisionLayers::new([Layer::Bots], [Layer::Ground, Layer::Constructed]), // Bots collides with ground, and constructed layers
            Friction::new(0.1),
            Restitution::new(0.2).with_combine_rule(CoefficientCombine::Multiply),
            LinearDamping(0.1),
            AngularDamping(0.1),
            // TODO: Remove this once we have proper thrust components
            ExternalImpulse::new(transform.forward() * BOT_MOVEMENT_SPEED / 10.),
        ))
        .id();
    bot_entity
}

#[allow(clippy::needless_pass_by_value)]
fn bots_movement_system(
    time: Res<Time>,
    cells: Res<Cells>,
    map_data: Res<MapData>,
    mut bot_logic_update_timer: ResMut<BotLogicUpdateTimer>,
    mut query: Query<&mut Transform, With<Bot>>,
) {
    for mut transform in &mut query {
        if timer_just_finishes(time.delta(), &mut bot_logic_update_timer.0) {
            update_navigation_component(&mut transform, &cells, &map_data);
        }

        // let move_delta = transform.forward() * BOT_MOVEMENT_SPEED * time.delta_seconds();
        // transform.translation += move_delta;
    }
}

fn update_navigation_component(
    transform: &mut Mut<Transform>,
    cells: &Res<Cells>,
    map_data: &Res<MapData>,
) {
    if !can_move_in_direction(
        transform.translation,
        transform.forward(),
        cells,
        map_data.bounds,
    ) {
        if can_move_in_direction(
            transform.translation,
            transform.right(),
            cells,
            map_data.bounds,
        ) {
            transform.rotate_z(-FRAC_PI_2);
        } else if can_move_in_direction(
            transform.translation,
            transform.back(),
            cells,
            map_data.bounds,
        ) {
            transform.rotate_z(2. * -FRAC_PI_2);
        } else if can_move_in_direction(
            transform.translation,
            transform.left(),
            cells,
            map_data.bounds,
        ) {
            transform.rotate_z(3. * -FRAC_PI_2);
        }
    }
}

fn can_move_in_direction(
    position: Vec3,
    forward: Direction3d,
    cells: &Res<Cells>,
    bounds: IBounds3,
) -> bool {
    let cell_coords = CellCoords::from_game_coordinates(position);
    let cell_indices = cell_coords.as_cell_indices(bounds);

    if let Some(current_cell_type) = cells.get(cell_indices) {
        // TODO: the below will fail if the direction is equivalent in multiple axis but for now since we are not
        // interpolating/sleeping it will just move in a straight line
        let move_direction = (*forward / forward.abs().max_element()).as_ivec3();
        let forward_cell_coords = cell_indices + move_direction;
        if let Some(forward_cell_type) = cells.get(forward_cell_coords) {
            let result = match move_direction {
                IVec3::X => {
                    current_cell_type.is_open(cell::OPEN_POS_X)
                        && forward_cell_type.is_open(cell::OPEN_NEG_X)
                }
                IVec3::Y => {
                    current_cell_type.is_open(cell::OPEN_POS_Y)
                        && forward_cell_type.is_open(cell::OPEN_NEG_Y)
                }
                IVec3::Z => {
                    current_cell_type.is_open(cell::OPEN_POS_Z)
                        && forward_cell_type.is_open(cell::OPEN_NEG_Z)
                }
                IVec3::NEG_X => {
                    current_cell_type.is_open(cell::OPEN_NEG_X)
                        && forward_cell_type.is_open(cell::OPEN_POS_X)
                }
                IVec3::NEG_Y => {
                    current_cell_type.is_open(cell::OPEN_NEG_Y)
                        && forward_cell_type.is_open(cell::OPEN_POS_Y)
                }
                IVec3::NEG_Z => {
                    current_cell_type.is_open(cell::OPEN_NEG_Z)
                        && forward_cell_type.is_open(cell::OPEN_POS_Z)
                }
                _ => false,
            };
            // println!("{position:?} => {cell_coords:?} => {cell_indices:?} => ({forward:?} => {move_direction:?}) => {forward_cell_coords:?} => {current_cell_type:?} => {forward_cell_type:?} => {result}");
            return result;
        }
    }
    false
}
