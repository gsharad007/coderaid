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
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_translation(Vec3::new(0., 0., 4.)),
    //     ..Default::default()
    // });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4., 4., 4.)),
        point_light: PointLight {
            color: Color::RED,
            ..default()
        },
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-4., -4., 4.)),
        point_light: PointLight {
            color: Color::GREEN,
            ..default()
        },
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4., -4., 4.)),
        point_light: PointLight {
            color: Color::BLUE,
            ..default()
        },
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Sphere::new(1.).mesh().ico(5).unwrap()),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });

    const FLOOR_POSITIONS: [Vec3; 6] = [
        Vec3::new(0., 0., 1.),
        Vec3::new(-1., 0., 1.),
        Vec3::new(-2., 0., 1.),
        Vec3::new(1., 0., 1.),
        Vec3::new(2., 0., 1.),
        Vec3::new(3., 0., 1.),
    ];

    for pos in &FLOOR_POSITIONS {
        // Spawn floor plane mesh
        commands.spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(1., 1.)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            transform: Transform::from_translation(*pos).with_scale(Vec3::splat(0.9)),
            ..Default::default()
        });
    }

    // Spawn walls and roof meshes
    spawn_walls(&mut commands, &mut meshes, &mut materials);
    spawn_ceiling(&mut commands, &mut meshes, &mut materials);
}

/// Spawns four walls around the room
fn spawn_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    const WALL_POSITIONS: [Vec3; 6] = [
        Vec3::new(0., 1., 0.),
        Vec3::new(-1., 1., 0.),
        Vec3::new(-2., 1., 0.),
        Vec3::new(0., 1., 1.),
        Vec3::new(1., 1., 1.),
        Vec3::new(2., 1., 1.),
    ];

    for pos in &WALL_POSITIONS {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1., 1., 1.)),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_scale(Vec3::new(1., 2., 0.1) * 0.9).with_translation(*pos),
            ..Default::default()
        });
    }
}

/// Spawns a cylindrical ceiling above the room
fn spawn_ceiling(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cylinder::new(0.5, 1.)),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::X, FRAC_PI_2)),
        ..Default::default()
    });
}
