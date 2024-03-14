use bevy::prelude::*;
use derivative::Derivative;

#[derive(Debug)]
pub struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        _ = app.add_systems(Update, (movement_system, rotation_system));
        // .add_systems(Update, movement_system.with_run_criteria(FixedTimestep::step(0.02)))
        // .add_systems(Update, rotation_system.with_run_criteria(FixedTimestep::step(0.02)));
    }
}

#[derive(Bundle, Default, Debug)]
pub struct MoverBundle {
    pub linear_mover: LinearMover,
    pub angular_mover: AngularMover,
}

#[derive(Component, Clone, Copy, Debug, Derivative)]
#[derivative(Default)]
pub struct LinearMover {
    pub acceleration_rate: f32,
    pub friction_coefficient: f32,
    #[derivative(Default(value = "1.0"))]
    pub mass: f32,
    pub velocity: Vec3,
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct MoveToTarget {
    target: Vec3,
}

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct AngularMover {
    pub acceleration_rate: f32,
    pub damping: f32,
    pub mass: f32,
    pub velocity: Vec3,
}

#[derive(Component, Default, Clone, Copy, Debug)]
struct RotateToTarget {
    target: Quat,
}

#[allow(clippy::needless_pass_by_value)]
fn movement_system(
    time: Res<Time>,
    mut query: Query<(&MoveToTarget, &mut Transform, &mut LinearMover)>,
) {
    for (move_to, mut transform, mut linear_mover) in &mut query {
        let to_target = move_to.target - transform.translation;

        if to_target.length() < 0.1 {
            // Target is close enough
            linear_mover.velocity = Vec3::ZERO; // Stop moving
            continue;
        }

        // let direction = to_target.clamp_length_max(1.);
        let direction = to_target.normalize();
        let force = direction * linear_mover.acceleration_rate;
        let friction_force = linear_mover.velocity * linear_mover.friction_coefficient;
        let net_force = force - friction_force;
        let acceleration = net_force / linear_mover.mass;
        let velocity_delta = acceleration * time.delta_seconds();
        linear_mover.velocity += velocity_delta;

        let translation_delta = linear_mover.velocity * time.delta_seconds();
        transform.translation += translation_delta;
    }
}

#[allow(clippy::needless_pass_by_value)]
fn rotation_system(
    time: Res<Time>,
    mut query: Query<(&RotateToTarget, &mut Transform, &mut AngularMover)>,
) {
    for (rotate_to, mut transform, mut angular_mover) in &mut query {
        let current_rot = transform.rotation;
        let target_rot = rotate_to.target;
        let angle_difference = current_rot.angle_between(target_rot);

        if angle_difference < 0.05 {
            // Rotation is close enough
            angular_mover.velocity = Vec3::ZERO; // Stop rotating
            continue;
        }

        let delta_rot = target_rot * current_rot.inverse();
        let axis = delta_rot.mul_vec3(Vec3::Z).normalize_or_zero();
        let force = axis * angular_mover.acceleration_rate * angle_difference.signum();
        let friction_force = angular_mover.velocity * angular_mover.damping;
        let net_force = force - friction_force;
        let angular_acceleration = net_force / angular_mover.mass;
        angular_mover.velocity += angular_acceleration * time.delta_seconds();
        transform.rotation =
            Quat::from_axis_angle(axis, angular_mover.velocity.length() * time.delta_seconds())
                * current_rot;
    }
}
