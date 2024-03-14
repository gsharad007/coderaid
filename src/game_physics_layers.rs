use bevy_xpbd_3d::prelude::*;

#[derive(PhysicsLayer)]
pub enum Layer {
    Ground,
    Constructed,
    Bots,
}
