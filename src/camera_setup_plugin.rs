use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

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
