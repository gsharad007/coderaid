use core::f32::consts::FRAC_PI_2;

use bevy::pbr::light_consts::lumens;
use bevy::prelude::*;
use bevy::{
    app::{App, Plugin},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    render::{color::Color, mesh::Mesh},
    transform::components::Transform,
};
use bevy_xpbd_3d::components::RigidBody;
use bevy_xpbd_3d::prelude::*;

use crate::game_cells_plugin::cell;
use crate::game_cells_plugin::Cells;
use crate::game_coordinates_utils::{CellCoords, CELL_SIZE};
use crate::game_physics_layers::Layer;
use crate::game_scene_plugin::CellsSpawnedEvent;
use crate::game_setup_data::MapData;
use crate::ibounds3::IBounds3;

#[derive(Debug)]
pub struct VisualSceneElementsPlugin;

impl Plugin for VisualSceneElementsPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_systems(Update, create_scene);
    }
}

/// Creates the scene elements (floor, walls, ceiling)
fn create_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cells_spawned_reader: EventReader<CellsSpawnedEvent>,
    cells: Option<Res<Cells>>,
    map_data: Option<Res<MapData>>,
) {
    if let Some(cells) = cells {
        if let Some(map_data) = map_data {
            for CellsSpawnedEvent {} in cells_spawned_reader.read() {
                spawn_lighting_setup(&mut commands, &mut meshes, &mut materials);

                spawn_scene_cells(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &cells,
                    &map_data.bounds,
                );
            }
        }
    }
}

fn spawn_lighting_setup(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Add lighting
    // // Sun
    // commands.spawn(DirectionalLightBundle {
    //     transform: Transform::from_translation(Vec3::new(10., 50., 10.)),
    //     directional_light: DirectionalLight {
    //         color: Color::YELLOW,
    //         illuminance: lux::DIRECT_SUNLIGHT,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     ..default()
    // });

    // // Skylight
    // commands.spawn(DirectionalLightBundle {
    //     transform: Transform::from_translation(Vec3::new(-10., 50., -10.)),
    //     directional_light: DirectionalLight {
    //         color: Color::ALICE_BLUE,
    //         illuminance: lux::FULL_DAYLIGHT,
    //         ..default()
    //     },
    //     ..default()
    // });

    _ = commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(8., 8., 8.)),
        point_light: PointLight {
            color: Color::RED,
            range: 64.,
            // intensity: lumens::LUMENS_PER_LED_WATTS * 100.,
            ..default()
        },
        ..default()
    });

    _ = commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-8., 8., 8.)),
        point_light: PointLight {
            color: Color::GREEN,
            range: 64.,
            // intensity: lumens::LUMENS_PER_LED_WATTS * 100.,
            ..default()
        },
        ..default()
    });

    _ = commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(8., 8., -8.)),
        point_light: PointLight {
            color: Color::BLUE,
            range: 64.,
            // intensity: lumens::LUMENS_PER_LED_WATTS * 100.,
            ..default()
        },
        ..default()
    });

    _ = commands.spawn(PbrBundle {
        mesh: meshes.add(
            Sphere::new(0.125)
                .mesh()
                .ico(2)
                .expect("Failed to create icosphere"),
        ),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_precision_loss)]
fn spawn_scene_cells(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cells: &Cells,
    map_bounds: &IBounds3,
) {
    // // let cells_offset = -Vec3::ZERO * CELLS_POSITION_AXIS_ORIENTATION;
    // let cells_offset = AxisOrientedCellCoords::from_cell_coords(CellCoords::from_cell_indices(
    //     IVec3::ZERO,
    //     map_bounds,
    // ));

    for (level, z) in cells.array.iter().zip(0..) {
        for (row, y) in level.iter().zip(0..) {
            for (&cell_type, x) in row.iter().zip(0..) {
                let cell_coords = CellCoords::from_cell_indices(IVec3::new(x, y, z), map_bounds);
                // let cell_position = cell_coords_offsetted.as_cell_centered_visual_coordinates();
                let cell_position = cell_coords.as_game_coordinates();
                // println!("({x} {y} {z}) => {cell_coords:?} => {cell_position:?}");

                if cell_type == cell::EMPTY {
                    spawn_closed(commands, meshes, materials, cell_position);
                } else {
                    if cell_type.is_closed(cell::OPEN_NEG_X) {
                        spawn_wall_neg_x(commands, meshes, materials, cell_position);
                    }
                    if cell_type.is_closed(cell::OPEN_POS_X) {
                        spawn_wall_pos_x(commands, meshes, materials, cell_position);
                    }
                    if cell_type.is_closed(cell::OPEN_NEG_Y) {
                        spawn_wall_neg_y(commands, meshes, materials, cell_position);
                    }
                    if cell_type.is_closed(cell::OPEN_POS_Y) {
                        spawn_wall_pos_y(commands, meshes, materials, cell_position);
                    }
                    // if cell_type.is_closed(cell::OPEN_POS_Z) {
                    //     spawn_wall_pos_z(commands, meshes, materials, pos);
                    // }
                    if cell_type.is_closed(cell::OPEN_NEG_Z) {
                        spawn_wall_neg_z(commands, meshes, materials, cell_position);
                    }
                }
            }
        }
    }
}

fn spawn_wall_pos_x(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        position,
        Quat::from_rotation_y(-1. * FRAC_PI_2),
    );
}

fn spawn_wall_neg_x(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        position,
        Quat::from_rotation_y(-3. * FRAC_PI_2),
    );
}

fn spawn_wall_pos_y(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        position,
        Quat::from_rotation_x(-3. * FRAC_PI_2),
    );
}

fn spawn_wall_neg_y(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        position,
        Quat::from_rotation_x(-1. * FRAC_PI_2),
    );
}

#[allow(dead_code)]
fn spawn_wall_pos_z(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        position,
        Quat::from_rotation_x(-2. * FRAC_PI_2),
    );
}

fn spawn_wall_neg_z(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        position,
        Quat::from_rotation_x(0. * FRAC_PI_2),
    );
}

const WALL_THICKNESS: f32 = 0.1; // Thickness of the wall
const WALL_MASS_DENSITY_SCALE: f32 = 1.0;

fn spawn_closed(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    _ = commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::splat(CELL_SIZE - WALL_THICKNESS))),
        material: materials.add(Color::rgb(0.2, 0.1, 0.0)),
        transform: Transform::from_translation(position),
        ..default()
    });
}

fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    rotation: Quat,
) {
    let offset = rotation.mul_vec3(Vec3::new(0.0, 0.0, -(0.5 - WALL_THICKNESS)));

    let size = Vec3::new(
        WALL_THICKNESS.mul_add(-2., CELL_SIZE),
        WALL_THICKNESS.mul_add(-2., CELL_SIZE),
        WALL_THICKNESS,
    );
    let collider = Collider::cuboid(size.x, size.y, size.z);

    _ = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::from_size(size)),
                material: materials.add(Color::rgb(0.9, 0.9, 0.9)),
                transform: Transform::from_translation(position + offset).with_rotation(rotation),
                ..default()
            },
            RigidBody::Static,
            MassPropertiesBundle::new_computed(&collider, WALL_MASS_DENSITY_SCALE),
            collider,
            // Wals collides with everything ground, and constructed layers
            CollisionLayers::new(
                [Layer::Constructed],
                [Layer::Ground, Layer::Constructed, Layer::Bots],
            ),
        ))
        .with_children(|parent| {
            _ = parent.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::rgb(0.9, 0.8, 1.0),
                    intensity: lumens::LUMENS_PER_LED_WATTS * 2.,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, WALL_THICKNESS),
                ..default()
            });
        });
}
