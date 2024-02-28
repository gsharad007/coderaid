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

use itertools::Itertools;

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
        mesh: meshes.add(
            Sphere::new(1.)
                .mesh()
                .ico(5)
                .expect("Failed to create icosphere"),
        ),
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

pub mod cell {
    // #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, BitOr)]
    // pub struct Type(u8);
    pub type Type = u8;

    pub const EMPTY: Type = 0b0;
    pub const OPEN_TOP: Type = 0b1;
    pub const OPEN_BOTTOM: Type = 0b10;
    pub const OPEN_LEFT: Type = 0b100;
    pub const OPEN_RIGHT: Type = 0b1000;
}

#[derive(Component, Debug)]
pub struct Map {
    pub cells: Vec<Vec<cell::Type>>,
}

impl Map {
    #[allow(clippy::non_ascii_literal)]
    pub fn from_string(map_string: &str) -> Self {
        let cells = map_string
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| -> cell::Type {
                        match c {
                            // '█' => cell::EMPTY,
                            '╨' => cell::OPEN_TOP,
                            '╥' => cell::OPEN_BOTTOM,
                            '╞' => cell::OPEN_RIGHT,
                            '╡' => cell::OPEN_LEFT,
                            '║' => cell::OPEN_TOP | cell::OPEN_BOTTOM,
                            '═' => cell::OPEN_LEFT | cell::OPEN_RIGHT,
                            '╝' => cell::OPEN_TOP | cell::OPEN_LEFT,
                            '╚' => cell::OPEN_TOP | cell::OPEN_RIGHT,
                            '╗' => cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                            '╔' => cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                            '╠' => cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                            '╣' => cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                            '╩' => cell::OPEN_TOP | cell::OPEN_LEFT | cell::OPEN_RIGHT,
                            '╦' => cell::OPEN_BOTTOM | cell::OPEN_LEFT | cell::OPEN_RIGHT,
                            '╬' => {
                                cell::OPEN_TOP
                                    | cell::OPEN_LEFT
                                    | cell::OPEN_RIGHT
                                    | cell::OPEN_BOTTOM
                            }
                            _ => cell::EMPTY,
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self { cells }
    }
}

#[cfg(test)]
mod test_map_load_string {
    use super::*;

    #[test]
    fn test_map_load_string_4x4() {
        #[allow(clippy::non_ascii_literal)]
        let map_string = "
╞═╦╗
╞═╬╣
██║║
╞═╩╝
";

        let map = Map::from_string(map_string);

        // println!("{}", map_string);
        // println!("{:?}", map.cells);

        assert_eq!(map.cells[0][0], cell::OPEN_RIGHT);
        assert_eq!(map.cells[0][1], cell::OPEN_LEFT | cell::OPEN_RIGHT);
        assert_eq!(
            map.cells[0][2],
            cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_BOTTOM
        );
        assert_eq!(map.cells[0][3], cell::OPEN_LEFT | cell::OPEN_BOTTOM);

        assert_eq!(map.cells[1][0], cell::OPEN_RIGHT);
        assert_eq!(map.cells[1][1], cell::OPEN_LEFT | cell::OPEN_RIGHT);
        assert_eq!(
            map.cells[1][2],
            cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_BOTTOM | cell::OPEN_TOP
        );
        assert_eq!(
            map.cells[1][3],
            cell::OPEN_LEFT | cell::OPEN_BOTTOM | cell::OPEN_TOP
        );

        assert_eq!(map.cells[2][0], cell::EMPTY);
        assert_eq!(map.cells[2][1], cell::EMPTY);
        assert_eq!(map.cells[2][2], cell::OPEN_TOP | cell::OPEN_BOTTOM);
        assert_eq!(map.cells[2][3], cell::OPEN_TOP | cell::OPEN_BOTTOM);

        assert_eq!(map.cells[3][0], cell::OPEN_RIGHT);
        assert_eq!(map.cells[3][1], cell::OPEN_LEFT | cell::OPEN_RIGHT);
        assert_eq!(
            map.cells[3][2],
            cell::OPEN_LEFT | cell::OPEN_RIGHT | cell::OPEN_TOP
        );
        assert_eq!(map.cells[3][3], cell::OPEN_LEFT | cell::OPEN_TOP);
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_map_load_string_8x8() {
        #[allow(clippy::non_ascii_literal)]
        let map_string = "
╞═╦╗╔╦╩╡
╞═╬╣╠╬╦╡
██║║╠╣║█
╞═╩╝╠╣║█
╔╗╔╗╚╝║█
╝╚╝╚╗╔╝█
████╚╝██
████╔╗██
";

        let map = Map::from_string(map_string);

        // println!("{}", map_string);
        // println!("{:?}", map.cells);

        // ╞═╦╗╔╦╩╡
        assert_eq!(
            map.cells[0],
            [
                cell::OPEN_RIGHT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_BOTTOM,
                cell::OPEN_LEFT | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_TOP,
                cell::OPEN_LEFT
            ]
        );

        // ╞═╬╣╠╬╦╡
        assert_eq!(
            map.cells[1],
            [
                cell::OPEN_RIGHT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_LEFT | cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_BOTTOM,
                cell::OPEN_LEFT
            ]
        );

        // ██║║╠╣║█
        assert_eq!(
            map.cells[2],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::EMPTY
            ]
        );

        // ╞═╩╝╠╣║█
        assert_eq!(
            map.cells[3],
            [
                cell::OPEN_RIGHT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT,
                cell::OPEN_RIGHT | cell::OPEN_LEFT | cell::OPEN_TOP,
                cell::OPEN_LEFT | cell::OPEN_TOP,
                cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::EMPTY
            ]
        );

        // ╔╗╔╗╚╝║█
        assert_eq!(
            map.cells[4],
            [
                cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_BOTTOM,
                cell::EMPTY
            ]
        );

        // ╝╚╝╚╗╔╝█
        assert_eq!(
            map.cells[5],
            [
                cell::OPEN_TOP | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_LEFT,
                cell::OPEN_TOP | cell::OPEN_RIGHT,
                cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_LEFT,
                cell::EMPTY
            ]
        );

        // ████╚╝██
        assert_eq!(
            map.cells[6],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_TOP | cell::OPEN_RIGHT,
                cell::OPEN_TOP | cell::OPEN_LEFT,
                cell::EMPTY,
                cell::EMPTY,
            ]
        );

        // ████╔╗██
        assert_eq!(
            map.cells[7],
            [
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::EMPTY,
                cell::OPEN_BOTTOM | cell::OPEN_RIGHT,
                cell::OPEN_BOTTOM | cell::OPEN_LEFT,
                cell::EMPTY,
                cell::EMPTY,
            ]
        );
    }
}
