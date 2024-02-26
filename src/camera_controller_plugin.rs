use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::{
    app::{App, Plugin, Update},
    ecs::system::Res,
};

use crate::camera_setup_plugin::{CameraLooking, CameraTarget};

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
