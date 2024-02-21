use bevy::app::*;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::render::camera::OrthographicProjection;
use bevy::utils::default;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraSetupPlugin)
        .add_plugins(SceneElementsPlugin)
        // .insert_resource(WindowDescriptor {
        //     title: "CodeRaid".to_string(),
        //     ..default()
        // })
        .run();
}

#[derive(Debug)]
pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClearColor>()
            .add_systems(Startup, setup_ortho_camera);
    }
}

/// Sets up an orthographic camera with default parameters
fn setup_ortho_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(4., 3., 4.).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scale: 0.01,
            // viewport_origin: WindowOrigin::Center,
            // scaling_mode: ScalingMode::FixedVertical,
            ..default()
        }
            .into(),
        ..Default::default()
    });
}

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

    const FLOOR_POSITIONS: [Vec3; 6] = [
        Vec3::new(0., 0., 1.),
        Vec3::new(-1., 0., 1.),
        Vec3::new(-2., 0., 1.),
        Vec3::new(1., 0., 1.),
        Vec3::new(2., 0., 1.),
        Vec3::new(3., 0., 1.),
    ];

    for pos in FLOOR_POSITIONS.iter() {
        // Spawn floor plane mesh
        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(1.).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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

    for pos in WALL_POSITIONS.iter() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_scale(Vec3::new(1., 2., 0.1) * Vec3::splat(0.9))
                .with_translation(*pos),
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
        mesh: meshes.add(Mesh::from(shape::Cylinder {
            radius: 0.5,
            height: 1.,
            // segments: 12,
            ..default()
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_rotation(Quat::from_axis_angle(
            Vec3::X,
            std::f32::consts::FRAC_PI_2,
        )),
        ..Default::default()
    });
}
