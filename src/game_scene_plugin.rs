use core::f32::consts::FRAC_PI_2;

use bevy::pbr::light_consts::lumens;
use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::{light_consts::lux, PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    render::{color::Color, mesh::Mesh},
    transform::components::Transform,
};
use bevy::{log, prelude::*};

use crate::game_cells_plugin::cell;
use crate::game_cells_plugin::Cells;
use crate::game_coordinates_utils::{AxisOrientedCellIndices, CellIndices, CELL_SIZE};
use crate::game_setup_data::MapData;
use crate::ibounds3::IBounds3;

#[derive(Debug)]
pub struct SceneElementsPlugin;

impl Plugin for SceneElementsPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_systems(Startup, create_scene);
    }
}

/// Creates the scene elements (floor, walls, ceiling)
fn create_scene(
    mut commands: Commands,
    mut map_data: ResMut<MapData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    spawn_map_cells(&mut commands, &mut map_data, &mut meshes, &mut materials);
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_precision_loss)]
fn spawn_map_cells(
    commands: &mut Commands,
    map_data: &mut ResMut<MapData>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
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

    map_data.bounds = IBounds3::new(IVec3::ZERO, cells.size);
    println!("Map bounds: {:?}", map_data.bounds);

    // let cells_offset = -Vec3::ZERO * CELLS_POSITION_AXIS_ORIENTATION;
    let cells_offset =
        AxisOrientedCellIndices::from_cell_indices(CellIndices::from_ivec3(map_data.bounds.min));

    for (level, z) in cells.array.iter().zip(0..) {
        for (row, y) in level.iter().zip(0..) {
            for (&cell_type, x) in row.iter().zip(0..) {
                let cell_indices_offsetted =
                    AxisOrientedCellIndices::from_cell_indices(CellIndices::new(x, y, z))
                        + cells_offset;
                let cell_position = cell_indices_offsetted.as_cell_centered_visual_coordinates();

                if cell_type == cell::EMPTY {
                    spawn_closed(commands, meshes, materials, cell_position);
                } else {
                    if cell_type & cell::OPEN_POS_X != cell::OPEN_POS_X {
                        spawn_wall_pos_x(commands, meshes, materials, cell_position);
                    }
                    if cell_type & cell::OPEN_NEG_X != cell::OPEN_NEG_X {
                        spawn_wall_neg_x(commands, meshes, materials, cell_position);
                    }
                    if cell_type & cell::OPEN_POS_Y != cell::OPEN_POS_Y {
                        spawn_wall_pos_y(commands, meshes, materials, cell_position);
                    }
                    if cell_type & cell::OPEN_NEG_Y != cell::OPEN_NEG_Y {
                        spawn_wall_neg_y(commands, meshes, materials, cell_position);
                    }
                    // if cell_type & cell::OPEN_POS_Z != cell::OPEN_POS_Z {
                    //     spawn_wall_pos_z(commands, meshes, materials, pos);
                    // }
                    if cell_type & cell::OPEN_NEG_Z != cell::OPEN_NEG_Z {
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
        Quat::from_rotation_x(-1. * FRAC_PI_2),
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
        Quat::from_rotation_x(-3. * FRAC_PI_2),
    );
}

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

    _ = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(
                CELL_SIZE - WALL_THICKNESS,
                CELL_SIZE - WALL_THICKNESS,
                WALL_THICKNESS,
            ))),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9)),
            transform: Transform::from_translation(position + offset).with_rotation(rotation),
            ..default()
        })
        .with_children(|parent| {
            _ = parent.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::rgb(0.8, 0.8, 1.0),
                    intensity: lumens::LUMENS_PER_LED_WATTS * 6.,
                    ..default()
                },
                ..default()
            });
        });
}
