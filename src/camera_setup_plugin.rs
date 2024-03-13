use bevy::app::{App, Plugin};
use bevy::prelude::*;

#[derive(Debug)]
pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .init_resource::<ClearColor>()
            .add_systems(Startup, setup_perspective_camera_3d);
    }
}

// /// Sets up an orthographic camera with default parameters
// fn setup_ortho_camera(mut commands: Commands) {
//     commands.spawn(
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
//         }
//     );
// }

/// Sets up a perspective camera with default parameters
fn setup_perspective_camera_3d(mut commands: Commands) {
    _ = commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(8., -8., 16.).looking_at(Vec3::ZERO, Vec3::Z),
        projection: PerspectiveProjection::default().into(),
        ..default()
    });
}
