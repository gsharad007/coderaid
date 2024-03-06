use core::f32::consts::FRAC_PI_2;

use bevy::pbr::light_consts::lumens;
use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::{light_consts::lux, PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    render::{color::Color, mesh::Mesh},
    transform::components::Transform,
};

use crate::game_cells_plugin::cell;
use crate::game_cells_plugin::Cells;

#[derive(Debug)]
pub struct SceneElementsPlugin;

impl Plugin for SceneElementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_scene);
    }
}

/// Creates the scene elements (floor, walls, ceiling)
fn create_scene(
    mut commands: Commands,
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

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(8., 8., 8.)),
        point_light: PointLight {
            color: Color::RED,
            range: 64.,
            // intensity: lumens::LUMENS_PER_LED_WATTS * 100.,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-8., 8., 8.)),
        point_light: PointLight {
            color: Color::GREEN,
            range: 64.,
            // intensity: lumens::LUMENS_PER_LED_WATTS * 100.,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(8., 8., -8.)),
        point_light: PointLight {
            color: Color::BLUE,
            range: 64.,
            // intensity: lumens::LUMENS_PER_LED_WATTS * 100.,
            ..default()
        },
        ..default()
    });

    commands.spawn(PbrBundle {
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

    spawn_map_cells(&mut commands, &mut meshes, &mut materials);
}

const CELLS_POSITION_AXIS_ORIENTATION: IVec3 = IVec3::new(1, -1, -1);

#[allow(clippy::cast_precision_loss)]
fn spawn_map_cells(
    commands: &mut Commands,
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

    #[allow(clippy::cast_possible_wrap)]
    // let cells_offset = -Vec3::ZERO * CELLS_POSITION_AXIS_ORIENTATION;
    let cells_offset = -IVec3::new(
        (cells.x / 2) as i32,
        (cells.y / 2) as i32,
        (cells.z / 2) as i32,
    ) * CELLS_POSITION_AXIS_ORIENTATION;

    for (level, z) in cells.array.iter().zip(0..) {
        for (row, y) in level.iter().zip(0..) {
            for (&cell_type, x) in row.iter().zip(0..) {
                let pos = (IVec3::new(x, y, z) * CELLS_POSITION_AXIS_ORIENTATION) + cells_offset;

                if cell_type == cell::EMPTY {
                    spawn_closed(commands, meshes, materials, pos);
                } else {
                    if cell_type & cell::OPEN_POS_X != cell::OPEN_POS_X {
                        spawn_wall_pos_x(commands, meshes, materials, pos);
                    }
                    if cell_type & cell::OPEN_NEG_X != cell::OPEN_NEG_X {
                        spawn_wall_neg_x(commands, meshes, materials, pos);
                    }
                    if cell_type & cell::OPEN_POS_Y != cell::OPEN_POS_Y {
                        spawn_wall_pos_y(commands, meshes, materials, pos);
                    }
                    if cell_type & cell::OPEN_NEG_Y != cell::OPEN_NEG_Y {
                        spawn_wall_neg_y(commands, meshes, materials, pos);
                    }
                    // if cell_type & cell::OPEN_POS_Z != cell::OPEN_POS_Z {
                    //     spawn_wall_pos_z(commands, meshes, materials, pos);
                    // }
                    if cell_type & cell::OPEN_NEG_Z != cell::OPEN_NEG_Z {
                        spawn_wall_neg_z(commands, meshes, materials, pos);
                    }
                }
            }
        }
    }
}

const CELL_VISUAL_OFFSET: Vec3 = Vec3::new(0.5, -0.5, 0.5);
const CELL_SIZE: f32 = 1.0; // Assuming the cell size is 1 unit
const WALL_THICKNESS: f32 = 0.1; // Thickness of the wall

fn spawn_closed(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: IVec3,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::ONE - WALL_THICKNESS)),
        material: materials.add(Color::rgb(0.2, 0.1, 0.0)),
        transform: Transform::from_translation(pos.as_vec3() + CELL_VISUAL_OFFSET),
        ..default()
    });
}

fn spawn_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::new(1., 0.01, 1.) * 0.9)),
        material: materials.add(Color::rgb(0.7, 0.7, 0.7)),
        transform: Transform::from_translation(pos),
        ..default()
    });
}

fn spawn_ceiling(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(1., 0.01, 1.) * 0.2)),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9)),
            transform: Transform::from_translation(pos + Vec3::new(0., 1., 0.)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::rgb(0.9, 0.9, 1.0),
                    intensity: lumens::LUMENS_PER_LED_WATTS * 6.,
                    ..default()
                },
                ..default()
            });
        });
}

fn spawn_wall_pos_x(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: IVec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        pos,
        Quat::from_rotation_y(-1. * FRAC_PI_2),
    );
}

fn spawn_wall_neg_x(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: IVec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        pos,
        Quat::from_rotation_y(-3. * FRAC_PI_2),
    );
}

fn spawn_wall_pos_y(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: IVec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        pos,
        Quat::from_rotation_x(-1. * FRAC_PI_2),
    );
}

fn spawn_wall_neg_y(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: IVec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        pos,
        Quat::from_rotation_x(-3. * FRAC_PI_2),
    );
}

fn spawn_wall_pos_z(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: IVec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        pos,
        Quat::from_rotation_x(-2. * FRAC_PI_2),
    );
}

fn spawn_wall_neg_z(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: IVec3,
) {
    spawn_wall(
        commands,
        meshes,
        materials,
        pos,
        Quat::from_rotation_x(0. * FRAC_PI_2),
    );
}

fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cell_indices: IVec3,
    rotation: Quat,
) {
    let offset = rotation.mul_vec3(Vec3::new(0.0, 0.0, -(0.5 - WALL_THICKNESS)));
    let position = (cell_indices.as_vec3() + CELL_VISUAL_OFFSET) * CELL_SIZE;

    commands
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
            parent.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::rgb(0.8, 0.8, 1.0),
                    intensity: lumens::LUMENS_PER_LED_WATTS * 6.,
                    ..default()
                },
                ..default()
            });
        });
}
