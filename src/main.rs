use core::f32::consts::FRAC_PI_2;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::utils::default;
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraSetupPlugin)
        .add_plugins(CameraControllerPlugin)
        .add_plugins(SceneElementsPlugin)
        // .insert_resource(WindowDescriptor {
        //     title: "CodeRaid".to_string(),
        //     ..default()
        // })
        .run();
}

#[derive(Debug)]
pub struct CameraSetupPlugin;

#[derive(Component)]
pub struct CameraTarget {
    pub target: Vec3,
}

impl Default for CameraTarget {
    fn default() -> Self {
        Self { target: Vec3::ZERO }
    }
}

#[derive(Component)]
pub struct CameraLooking {
    pub look_from: Vec3,
    pub up: Vec3,
}

impl Default for CameraLooking {
    fn default() -> Self {
        Self {
            look_from: Vec3::ONE,
            up: Vec3::Y,
        }
    }
}

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClearColor>()
            .add_systems(Startup, setup_perspective_camera_3d)
            .add_systems(Update, update_camera_target);
    }
}

// /// Sets up an orthographic camera with default parameters
// fn setup_ortho_camera(mut commands: Commands) {
//     commands.spawn((
//         Camera3dBundle {
//             transform: Transform::from_xyz(4., 3., 4.).looking_at(Vec3::ZERO, Vec3::Y),
//             projection: OrthographicProjection {
//                 scale: 0.01,
//                 // viewport_origin: WindowOrigin::Center,
//                 // scaling_mode: ScalingMode::FixedVertical,
//                 ..default()
//             }
//             .into(),
//             ..default()
//         },
//         CameraTarget {
//             look_from: Vec3::splat(16.),
//             ..default()
//         },
//     ));
// }

/// Sets up a perspective camera with default parameters
fn setup_perspective_camera_3d(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10., 12., 10.).looking_at(Vec3::ZERO, Vec3::Y),
            projection: PerspectiveProjection::default().into(),
            ..default()
        },
        CameraTarget::default(),
        CameraLooking {
            look_from: Vec3::splat(16.),
            ..default()
        },
    ));
}

/// Update the Camera using the `CameraTarget`
fn update_camera_target(
    mut query: Query<(&mut Transform, &CameraTarget, &CameraLooking), With<Camera>>,
) {
    for (mut transform, camera_target, camera_looking) in &mut query {
        let camera_locations = camera_target.target + camera_looking.look_from;
        let camera_looking_to = -camera_looking.look_from;
        let camera_up = camera_looking.up;
        *transform =
            Transform::from_translation(camera_locations).looking_to(camera_looking_to, camera_up);
    }
}

#[derive(Debug)]
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (camera_panning_system, camera_orbiting_system));
    }
}

const CAMERA_PANNING_SPEED: f32 = 10.;
const CAMERA_ORBITING_SPEED: f32 = 10.;

#[allow(clippy::needless_pass_by_value)]
fn camera_panning_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // mut keyboard_events: EventReader<KeyboardInput>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut CameraTarget, &Transform), With<Camera>>,
) {
    let translation = Vec2::ZERO;

    // Keyboard control
    // // This unfortunately falls under the system key repeat control causing the movement to be jerky (move + pause + move continuesly)
    // let translation = keyboard_events
    //     .read()
    //     .filter_map(
    //         |keyboard_input| match keyboard_input.state == ButtonState::Pressed {
    //             true => Some(keyboard_input.key_code),
    //             false => None,
    //         },
    //     )
    //     .fold(Vec2::ZERO, |acc, key| {
    //         acc + match key {
    //             Some(KeyCode::W) => Vec2::new(0., 1.),
    //             Some(KeyCode::S) => -Vec2::new(0., 1.),
    //             Some(KeyCode::A) => -Vec2::new(1., 0.),
    //             Some(KeyCode::D) => Vec2::new(1., 0.),
    //             _ => Vec2::ZERO,
    //         }
    //     });
    let translation = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        translation
    } else {
        keyboard_input.get_pressed().fold(translation, |acc, &key| {
            acc + match key {
                KeyCode::KeyW => Vec2::new(0., 1.),
                KeyCode::KeyS => -Vec2::new(0., 1.),
                KeyCode::KeyA => -Vec2::new(1., 0.),
                KeyCode::KeyD => Vec2::new(1., 0.),
                _ => Vec2::ZERO,
            }
        })
    };

    // Mouse control
    let translation = if mouse_button_input.pressed(MouseButton::Middle) {
        mouse_motion_events
            .read()
            .fold(translation, |acc, &event| acc + event.delta)
    } else {
        translation
    };

    let translation = translation * (CAMERA_PANNING_SPEED * time.delta_seconds());

    for (mut camera_target, transform) in &mut query {
        let translation_right = transform.right().xz().normalize_or_zero();
        let translation_forward = transform
            .forward()
            .xz()
            .try_normalize()
            .unwrap_or_else(|| transform.up().xz().normalize_or_zero());
        let viewspace_translation =
            (translation_right * translation.x) + (translation_forward * translation.y);
        camera_target.target += Vec3::new(viewspace_translation.x, 0., viewspace_translation.y);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn camera_orbiting_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut CameraLooking, &Transform), With<Camera>>,
) {
    let delta = Vec2::ZERO;

    // Keyboard control
    let delta = if keyboard_input.pressed(KeyCode::ShiftLeft) {
        keyboard_input.get_pressed().fold(delta, |acc, &key| {
            acc + match key {
                KeyCode::KeyW => Vec2::new(1., 0.),
                KeyCode::KeyS => -Vec2::new(1., 0.),
                KeyCode::KeyA => -Vec2::new(0., 1.),
                KeyCode::KeyD => Vec2::new(0., 1.),
                _ => Vec2::ZERO,
            }
        })
    } else {
        delta
    };

    // Mouse control
    let delta = if mouse_button_input.pressed(MouseButton::Right) {
        mouse_motion_events
            .read()
            .fold(delta, |acc, &event| acc + event.delta)
    } else {
        delta
    };

    let delta = delta * (CAMERA_ORBITING_SPEED * time.delta_seconds());
    let pitch = delta.x;
    let yaw = delta.y;

    for (mut camera_looking, transform) in &mut query {
        let translation_right = transform.right().normalize_or_zero();

        let rotation =
            Quat::from_rotation_y(yaw).mul_quat(Quat::from_axis_angle(translation_right, pitch));

        camera_looking.look_from = rotation.mul_vec3(camera_looking.look_from);
        camera_looking.up = rotation.mul_vec3(camera_looking.up);
    }
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
