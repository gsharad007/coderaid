use core::f32::consts::FRAC_PI_2;
use core::time::Duration;

use bevy::prelude::*;
use bevy_prng::WyRand;
use bevy_rand::resource::GlobalEntropy;
use bevy_xpbd_3d::components::{CollisionLayers, RigidBody};
use bevy_xpbd_3d::plugins::collision::Collider;
use bevy_xpbd_3d::prelude::*;
use rand_core::RngCore;

use crate::game_cells_plugin::{cell, Cells};
use crate::game_coordinates_utils::CellCoords;
use crate::game_physics_layers::Layer;
use crate::game_setup_data::MapData;
use crate::ibounds3::IBounds3;

const BOT_SPAWNING_INTERVAL: f32 = 0.5;
const BOT_LOGIC_UPDATE_INTERVAL: f32 = 0.5;

const BOT_MOVEMENT_SPEED: f32 = 0.1;
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
    let collider = Collider::cylinder(0.2, 0.25);
    let bot_entity = commands
        .spawn((
            Bot {},
            SpatialBundle::from_transform(transform),
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&collider, BOT_MASS_DENSITY_SCALE),
            collider,
            CollisionLayers::new([Layer::Bots], [Layer::Ground, Layer::Constructed]), // Bots collides with ground, and constructed layers
            Friction::new(0.1),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Multiply),
            LinearDamping(0.2),
            AngularDamping(0.2),
        ))
        .id();
    bot_entity
}

#[allow(clippy::needless_pass_by_value)]
fn bots_movement_system(
    time: Res<Time>,
    cells: Res<Cells>,
    map_data: Res<MapData>,
    mut commands: Commands,
    mut bot_logic_update_timer: ResMut<BotLogicUpdateTimer>,
    mut query: Query<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            Entity,
        ),
        With<Bot>,
    >,
) {
    for (mut transform, mut linear_velocity, mut angular_velocity, entity) in &mut query {
        if timer_just_finishes(time.delta(), &mut bot_logic_update_timer.0) {
            update_brakes_level0_component(
                *transform,
                &mut linear_velocity,
                &mut angular_velocity,
                &cells,
                &map_data,
            );
            update_navigation_level2_component(&mut transform, &cells, &map_data);
            update_forward_thruster_level1_component(
                commands.reborrow(),
                *transform,
                entity,
                &cells,
                &map_data,
            );
        }

        // let move_delta = transform.forward() * BOT_MOVEMENT_SPEED * time.delta_seconds();
        // transform.translation += move_delta;
    }
}

fn update_brakes_level0_component(
    transform: Transform,
    linear_velocity: &mut Mut<LinearVelocity>,
    angular_velocity: &mut Mut<AngularVelocity>,
    cells: &Res<Cells>,
    map_data: &Res<MapData>,
) {
    if !can_move_in_direction_vector(
        transform.translation,
        transform.forward(),
        cells,
        &map_data.bounds,
    ) {
        linear_velocity.0 = Vec3::ZERO;
        angular_velocity.0 = Vec3::ZERO;
    }
}

fn update_forward_thruster_level1_component(
    mut commands: Commands,
    transform: Transform,
    entity: Entity,
    cells: &Res<Cells>,
    map_data: &Res<MapData>,
) {
    if can_move_in_direction_vector(
        transform.translation,
        transform.forward(),
        cells,
        &map_data.bounds,
    ) {
        let impulse = ExternalImpulse::new(transform.forward() * BOT_MOVEMENT_SPEED / 10.);
        _ = commands.entity(entity).insert(impulse);
    }
}

#[allow(dead_code)]
fn update_navigation_level1_component(
    transform: &mut Mut<Transform>,
    cells: &Res<Cells>,
    map_data: &Res<MapData>,
) {
    let bounds = &map_data.bounds;

    if !can_move_in_direction_vector(transform.translation, transform.forward(), cells, bounds) {
        if can_move_in_direction_vector(transform.translation, transform.right(), cells, bounds) {
            transform.rotate_z(-FRAC_PI_2);
        } else if can_move_in_direction_vector(
            transform.translation,
            transform.back(),
            cells,
            bounds,
        ) {
            transform.rotate_z(2. * -FRAC_PI_2);
        } else if can_move_in_direction_vector(
            transform.translation,
            transform.left(),
            cells,
            bounds,
        ) {
            transform.rotate_z(3. * -FRAC_PI_2);
        }
    }
}

#[allow(dead_code)]
fn update_navigation_level2_component(
    transform: &mut Mut<Transform>,
    cells: &Res<Cells>,
    map_data: &Res<MapData>,
) {
    let bounds = &map_data.bounds;
    let game_coords = transform.translation;
    let src_cell_coords = CellCoords::from_game_coordinates(game_coords);
    let src_cell_indices = src_cell_coords.as_cell_indices(bounds);

    let directions = [
        transform.forward(),
        transform.right(),
        transform.left(),
        transform.back(),
    ];

    for (idx, &direction) in directions.iter().enumerate() {
        let move_direction = calculate_move_direction_from_direction_vector(direction);
        let dst_cell_indices = calculate_destination_cell_indices_from_move_direction(
            src_cell_indices,
            move_direction,
        );
        if can_move_to_cell(src_cell_indices, dst_cell_indices, cells) {
            let up = transform.up();
            let dst_cell_coords =
                CellCoords::from_cell_indices(dst_cell_indices, bounds).as_game_coordinates();
            let dst_cell_direction = dst_cell_coords - game_coords;
            let to = (dst_cell_direction / 4.) + move_direction.as_vec3();
            transform.look_to(to, *up);

            println!("[{idx}] world_pos: {game_coords} => src: {src_cell_coords} => src_cell_indices: {src_cell_indices}, direction: {direction:?} => move_direction: {move_direction} => dst_cell_indices: {dst_cell_indices} => look_to: {dst_cell_direction}");

            break;
        }
    }
}

fn can_move_in_direction_vector(
    position: Vec3,
    forward: Direction3d,
    cells: &Cells,
    bounds: &IBounds3,
) -> bool {
    let src_cell_coords = CellCoords::from_game_coordinates(position);
    let src_cell_indices = src_cell_coords.as_cell_indices(bounds);

    let move_direction = calculate_move_direction_from_direction_vector(forward);

    debug_assert_eq!(move_direction.abs().max_element(), 1, "move_direction: {forward:?} => {move_direction:?} currently only support moves that can be adjucent to the source cell!");
    debug_assert_eq!(move_direction.length_squared(), 1, "move_direction: {forward:?} => {move_direction:?} currently only support moves that can be adjucent to the source cell!");

    can_move_in_direction(src_cell_indices, move_direction, cells)
}

fn can_move_to_cell(src_cell_indices: IVec3, dst_cell_indices: IVec3, cells: &Cells) -> bool {
    let move_direction = dst_cell_indices - src_cell_indices;

    debug_assert_eq!(move_direction.abs().max_element(), 1, "move_direction: {dst_cell_indices:?} - {src_cell_indices:?} => {move_direction:?} currently only support moves that can be adjucent to the source cell!");
    debug_assert_eq!(move_direction.length_squared(), 1, "move_direction: {dst_cell_indices:?} - {src_cell_indices:?} => {move_direction:?} currently only support moves that can be adjucent to the source cell!");

    can_move_in_direction(src_cell_indices, move_direction, cells)
}

fn can_move_in_direction(src_cell_indices: IVec3, move_direction: IVec3, cells: &Cells) -> bool {
    debug_assert_eq!(move_direction.abs().max_element(), 1, "move_direction: {src_cell_indices:?} + {move_direction:?} currently only support moves that can be adjucent to the source cell!");
    debug_assert_eq!(move_direction.length_squared(), 1, "move_direction: {src_cell_indices:?} + {move_direction:?} currently only support moves that can be adjucent to the source cell!");

    let dst_cell_indices =
        calculate_destination_cell_indices_from_move_direction(src_cell_indices, move_direction);

    let src_cell_type = cells.get_or_open_all(src_cell_indices);
    let dst_cell_type = cells.get_or_open_all(dst_cell_indices);

    let result = match move_direction {
        IVec3::X => {
            src_cell_type.is_open(cell::OPEN_POS_X) && dst_cell_type.is_open(cell::OPEN_NEG_X)
        }
        IVec3::Y => {
            src_cell_type.is_open(cell::OPEN_POS_Y) && dst_cell_type.is_open(cell::OPEN_NEG_Y)
        }
        IVec3::Z => {
            src_cell_type.is_open(cell::OPEN_POS_Z) && dst_cell_type.is_open(cell::OPEN_NEG_Z)
        }
        IVec3::NEG_X => {
            src_cell_type.is_open(cell::OPEN_NEG_X) && dst_cell_type.is_open(cell::OPEN_POS_X)
        }
        IVec3::NEG_Y => {
            src_cell_type.is_open(cell::OPEN_NEG_Y) && dst_cell_type.is_open(cell::OPEN_POS_Y)
        }
        IVec3::NEG_Z => {
            src_cell_type.is_open(cell::OPEN_NEG_Z) && dst_cell_type.is_open(cell::OPEN_POS_Z)
        }
        _ => false,
    };
    println!("{src_cell_indices:?} + {move_direction:?}) => {dst_cell_indices:?} => {src_cell_type:?} => {dst_cell_type:?} => {result}");
    result
}

fn calculate_move_direction_from_direction_vector(forward: Direction3d) -> IVec3 {
    // TODO: the below will fail if the direction is equivalent in multiple axis but for now since we are not
    // interpolating/slerping it will just move in a straight line
    (*forward / forward.abs().max_element()).as_ivec3()
}

fn calculate_destination_cell_indices_from_move_direction(
    src_cell_indices: IVec3,
    move_direction: IVec3,
) -> IVec3 {
    src_cell_indices + move_direction
}
