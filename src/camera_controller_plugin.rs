use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Update},
    ecs::system::Res,
};

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_systems(PostStartup, setup_panning_orbiting_camera)
            .add_systems(
                Update,
                (camera_panning_system, camera_orbiting_system).before(update_camera_target),
            )
            .add_systems(Update, update_camera_target);
    }
}

#[derive(Component, Debug)]
pub struct CameraTarget {
    pub target: Vec3,
}

impl Default for CameraTarget {
    fn default() -> Self {
        Self { target: Vec3::ZERO }
    }
}

#[derive(Component, Debug)]
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

/// Sets up a perspective camera with default parameters
#[allow(clippy::needless_pass_by_value)]
fn setup_panning_orbiting_camera(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    for camera_entity in &query {
        _ = commands.entity(camera_entity).insert((
            CameraTarget::default(),
            CameraLooking {
                look_from: Vec3::splat(16.),
                ..default()
            },
        ));
    }
}

const CAMERA_PANNING_SPEED: f32 = 8.;
const CAMERA_ORBITING_SPEED: f32 = 4.;

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
        mouse_motion_events.read().fold(translation, |acc, &event| {
            acc + Vec2::new(-event.delta.x, event.delta.y)
        })
    } else {
        translation
    };

    if translation == Vec2::ZERO {
        return;
    }

    let translation = translation * (CAMERA_PANNING_SPEED * time.delta_seconds());

    for (mut camera_target, transform) in &mut query {
        let translation_up = transform.up().normalize_or_zero();
        let translation_right = transform.right().normalize_or_zero();
        let viewspace_translation =
            (translation_right * translation.x) + (translation_up * translation.y);
        camera_target.target += viewspace_translation;
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
                KeyCode::KeyW => Vec2::new(0., 1.),
                KeyCode::KeyS => -Vec2::new(0., 1.),
                KeyCode::KeyA => -Vec2::new(1., 0.),
                KeyCode::KeyD => Vec2::new(1., 0.),
                _ => Vec2::ZERO,
            }
        })
    } else {
        delta
    };

    // Mouse control
    let delta = if mouse_button_input.pressed(MouseButton::Right) {
        mouse_motion_events.read().fold(delta, |acc, &event| {
            acc + Vec2::new(-event.delta.x, event.delta.y)
        })
    } else {
        delta
    };

    if delta == Vec2::ZERO {
        return;
    }

    let delta = delta * (CAMERA_ORBITING_SPEED * time.delta_seconds());
    let yaw = delta.x;
    let pitch = delta.y;

    for (mut camera_looking, transform) in &mut query {
        let translation_up = transform.up().normalize_or_zero();
        let translation_right = transform.right().normalize_or_zero();

        let rotation = Quat::from_axis_angle(translation_up, yaw)
            .mul_quat(Quat::from_axis_angle(translation_right, -pitch));

        camera_looking.look_from = rotation.mul_vec3(camera_looking.look_from);
        camera_looking.up = rotation.mul_vec3(camera_looking.up);
    }
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
