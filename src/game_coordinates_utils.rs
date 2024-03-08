use core::f32::consts::FRAC_PI_2;
use derive_more::{Add, Neg, Sub};

use bevy::{
    math::{IVec3, Quat, Vec3},
    transform::components::Transform,
};

#[derive(Debug, Clone, Copy, Add, Sub)]
pub struct CellIndices {
    x: i32,
    y: i32,
    z: i32,
}

impl CellIndices {
    /// Creates a new
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub const fn from_ivec3(value: IVec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    pub const fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub const fn as_ivec3(&self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z)
    }

    #[inline]
    pub fn as_game_coordinates_transform(&self) -> Transform {
        Transform::from_translation(self.as_vec3()).with_rotation(Quat::from_rotation_x(FRAC_PI_2))
    }
}

impl From<IVec3> for CellIndices {
    #[inline]
    fn from(value: IVec3) -> Self {
        Self::from_ivec3(value)
    }
}

impl From<CellIndices> for IVec3 {
    #[inline]
    fn from(value: CellIndices) -> Self {
        value.as_ivec3()
    }
}

const CELLS_POSITION_AXIS_ORIENTATION: IVec3 = IVec3::new(1, -1, -1);

pub const CELL_SIZE: f32 = 1.0; // Assuming the cell size is 1 unit
const CELL_VISUAL_OFFSET: Vec3 = Vec3::new(0.5, -0.5, 0.5);

#[derive(Debug, Clone, Copy, Add, Sub, Neg)]
pub struct AxisOrientedCellIndices {
    x: i32,
    y: i32,
    z: i32,
}

impl AxisOrientedCellIndices {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0);

    /// Creates
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Creates with all elements set to `v`.
    #[inline]
    #[must_use]
    pub const fn splat(v: i32) -> Self {
        Self { x: v, y: v, z: v }
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    pub const fn from_ivec3(value: IVec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }

    #[inline]
    pub fn from_cell_indices(cell_indices: CellIndices) -> Self {
        let pos = cell_indices.as_ivec3() * CELLS_POSITION_AXIS_ORIENTATION;
        Self::from_ivec3(pos)
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    pub const fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    // /// Casts all elements of `self` to `f32`.
    // #[inline]
    // pub const fn as_ivec3(&self) -> IVec3 {
    //     IVec3::new(self.x, self.y, self.z)
    // }

    #[inline]
    pub fn as_cell_centered_visual_coordinates(&self) -> Vec3 {
        (self.as_vec3() + CELL_VISUAL_OFFSET) * CELL_SIZE
    }
}

impl From<IVec3> for AxisOrientedCellIndices {
    #[inline]
    #[must_use]
    fn from(ivec3: IVec3) -> Self {
        Self::from_ivec3(ivec3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_indices_new() {
        let cell_indices = CellIndices::new(1, 2, 3);
        assert_eq!(cell_indices.x, 1);
        assert_eq!(cell_indices.y, 2);
        assert_eq!(cell_indices.z, 3);
    }

    #[test]
    fn test_cell_indices_from_ivec3() {
        let ivec3 = IVec3::new(4, 5, 6);
        let cell_indices = CellIndices::from_ivec3(ivec3);
        assert_eq!(cell_indices.x, 4);
        assert_eq!(cell_indices.y, 5);
        assert_eq!(cell_indices.z, 6);
    }

    #[test]
    fn test_cell_indices_as_vec3() {
        let cell_indices = CellIndices::new(1, 2, 3);
        let vec3 = cell_indices.as_vec3();
        assert_eq!(vec3, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_cell_indices_as_ivec3() {
        let cell_indices = CellIndices::new(1, 2, 3);
        let ivec3 = cell_indices.as_ivec3();
        assert_eq!(ivec3, IVec3::new(1, 2, 3));
    }

    #[test]
    fn test_cell_indices_as_game_coordinates_transform() {
        let cell_indices = CellIndices::new(1, 2, 3);
        let transform = cell_indices.as_game_coordinates_transform();
        let expected_transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0))
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2));
        assert_eq!(transform, expected_transform);
    }

    #[test]
    fn test_axis_oriented_cell_indices_from_cell_indices() {
        let cell_indices = CellIndices::new(1, 2, 3);
        let axis_oriented_cell_indices = AxisOrientedCellIndices::from_cell_indices(cell_indices);
        assert_eq!(axis_oriented_cell_indices.x, 1);
        assert_eq!(axis_oriented_cell_indices.y, -3);
        assert_eq!(axis_oriented_cell_indices.z, -2);
    }

    #[test]
    fn test_axis_oriented_cell_indices_as_vec3() {
        let axis_oriented_cell_indices = AxisOrientedCellIndices::new(1, 2, 3);
        let vec3 = axis_oriented_cell_indices.as_vec3();
        assert_eq!(vec3, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_axis_oriented_cell_indices_as_cell_centered_visual_coordinates() {
        assert_eq!(
            AxisOrientedCellIndices::ZERO.as_cell_centered_visual_coordinates(),
            Vec3::new(0.5, -0.5, 0.5)
        );
        assert_eq!(
            AxisOrientedCellIndices::new(1, 2, 3).as_cell_centered_visual_coordinates(),
            Vec3::new(1.5, 1.5, 3.5)
        );
    }
}
