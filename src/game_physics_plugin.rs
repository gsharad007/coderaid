use bevy::app::FixedUpdate;
use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy_xpbd_3d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use bevy_xpbd_3d::resources::Gravity;

#[derive(Debug)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        _ = app
            .add_plugins(PhysicsPlugins::new(FixedUpdate))
            .insert_resource(Gravity(Vec3::ZERO)); // Disable Gravity since we are doing outer space experience for now

        enable_debug(app);
    }
}

fn enable_debug(_app: &mut App) {
    if cfg!(debug_assertions) {
        // _ = _app.add_plugins(PhysicsDebugPlugin::default());
    }
}
