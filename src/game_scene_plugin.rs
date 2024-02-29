use core::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
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
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0., 10., 0.)),
        point_light: PointLight {
            color: Color::WHITE,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4., 4., 4.)),
        point_light: PointLight {
            color: Color::RED,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-4., 4., 4.)),
        point_light: PointLight {
            color: Color::GREEN,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4., 4., -4.)),
        point_light: PointLight {
            color: Color::BLUE,
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

fn spawn_empty(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::ONE * 0.9)),
        material: materials.add(Color::rgb(0.2, 0.1, 0.0)),
        transform: Transform::from_translation(pos + Vec3::new(0., 0.5, 0.)),
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
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::new(1., 0.01, 1.) * 0.2)),
        material: materials.add(Color::rgb(0.9, 0.9, 0.9)),
        transform: Transform::from_translation(pos + Vec3::new(0., 1., 0.)),
        ..default()
    });
}

fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
    angle: f32,
) {
    let rotation = Quat::from_rotation_y(angle);
    let offset = rotation.mul_vec3(Vec3::new(0., 0.5, -0.5) * 0.9);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::new(1., 1., 0.1) * 0.9)),
        material: materials.add(Color::rgb(0.9, 0.9, 0.9)),
        transform: Transform::from_translation(pos + offset).with_rotation(rotation),
        ..default()
    });
}

fn spawn_wall_top(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) {
    spawn_wall(commands, meshes, materials, pos, 0.);
}

fn spawn_wall_bottom(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) {
    spawn_wall(commands, meshes, materials, pos, -2. * FRAC_PI_2);
}

fn spawn_wall_right(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) {
    spawn_wall(commands, meshes, materials, pos, -1. * FRAC_PI_2);
}

fn spawn_wall_left(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) {
    spawn_wall(commands, meshes, materials, pos, -3. * FRAC_PI_2);
}

#[allow(clippy::cast_precision_loss)]
fn spawn_map_cells(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    #[allow(clippy::non_ascii_literal)]
    let cells_string = "
╞═╦╗╔╦╩╡
╞═╬╣╠╬╦╡
██║║╠╣║█
╞═╩╝╠╣║█
╔╗╔╗╚╝║█
╝╚╝╚╗╔╝█
████╚╝██
████╔╗██
";

    let cells = Cells::from_string(cells_string);

    let x_offset = (-0.5f32).mul_add(
        cells
            .array
            .iter()
            .map(Vec::len)
            .max()
            .unwrap_or_default() as f32,
        0.5,
    );
    let z_offset = (-0.5f32).mul_add(cells.array.len() as f32, 0.5);

    for (z, row) in cells.array.iter().enumerate() {
        for (x, &cell_type) in row.iter().enumerate() {
            let pos = Vec3::new(x_offset + x as f32, 0., z_offset + z as f32);

            if cell_type == cell::EMPTY {
                spawn_empty(commands, meshes, materials, pos);
            } else {
                spawn_floor(commands, meshes, materials, pos);
                spawn_ceiling(commands, meshes, materials, pos);
                if cell_type & cell::OPEN_TOP != cell::OPEN_TOP {
                    spawn_wall_top(commands, meshes, materials, pos);
                }
                if cell_type & cell::OPEN_BOTTOM != cell::OPEN_BOTTOM {
                    spawn_wall_bottom(commands, meshes, materials, pos);
                }
                if cell_type & cell::OPEN_LEFT != cell::OPEN_LEFT {
                    spawn_wall_left(commands, meshes, materials, pos);
                }
                if cell_type & cell::OPEN_RIGHT != cell::OPEN_RIGHT {
                    spawn_wall_right(commands, meshes, materials, pos);
                }
            }
        }
    }
}
