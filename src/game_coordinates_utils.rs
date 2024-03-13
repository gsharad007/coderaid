use core::f32::consts::FRAC_PI_2;
use derive_more::{Add, Neg, Sub};

use bevy::{
    math::{IVec3, Quat, UVec3, Vec3},
    transform::components::Transform,
};

use crate::ibounds3::IBounds3;

#[derive(Debug, Clone, Copy, Add, Sub)]
pub struct CellCoords {
    x: i32,
    y: i32,
    z: i32,
}

impl CellCoords {
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

    pub fn from_cell_indices(value: IVec3, bounds: &IBounds3) -> Self {
        Self::from_ivec3(value + bounds.min)
    }

    #[inline]
    pub fn from_game_coordinates(value: Vec3) -> Self {
        Self::from_ivec3(value.as_ivec3())
    }

    /// Casts all elements of `self` to `f32`.
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    pub const fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Converts into an `IVec3`.
    ///
    /// # Example
    /// ```
    /// let vec = Vec3::new(1.0, 2.0, 3.0);
    /// let ivec = vec.as_ivec3();
    /// assert_eq!(ivec, IVec3::new(1, 2, 3));
    /// ```
    #[inline]
    pub const fn as_ivec3(&self) -> IVec3 {
        IVec3::new(self.x, self.y, self.z)
    }

    /// Converts into an cell indices vector of type `UVec3`.
    /// 1
    /// # Example
    /// ```
    /// let vec = Vec3::new(1.0, 2.0, 3.0);
    /// let ivec = vec.as_ivec3();
    /// assert_eq!(ivec, IVec3::new(1, 2, 3));
    /// ```
    #[inline]
    pub fn as_cell_indices(&self, bounds: IBounds3) -> IVec3 {
        debug_assert!(
            self.as_ivec3().cmpge(bounds.min).all() && self.as_ivec3().cmple(bounds.max).all(),
            "CellCoords {self:?} is out of bounds {bounds:?}"
        );
        self.as_ivec3() - bounds.min
    }

    /// Converts into an game coordinates.
    /// 1
    /// # Example
    /// ```
    /// let vec = Vec3::new(1.0, 2.0, 3.0);
    /// let ivec = vec.as_ivec3();
    /// assert_eq!(ivec, IVec3::new(1, 2, 3));
    /// ```
    #[inline]
    pub const fn as_game_coordinates(&self) -> Vec3 {
        self.as_vec3()
    }

    #[inline]
    pub fn as_game_coordinates_transform(&self) -> Transform {
        Transform::from_translation(self.as_game_coordinates())
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2))
    }
}

impl From<IVec3> for CellCoords {
    #[inline]
    fn from(value: IVec3) -> Self {
        Self::from_ivec3(value)
    }
}

impl From<CellCoords> for IVec3 {
    #[inline]
    fn from(value: CellCoords) -> Self {
        value.as_ivec3()
    }
}

const CELLS_POSITION_AXIS_ORIENTATION: IVec3 = IVec3::new(1, -1, -1);

pub const CELL_SIZE: f32 = 1.0; // Assuming the cell size is 1 unit
const CELL_VISUAL_OFFSET: Vec3 = Vec3::new(0.5, -0.5, 0.5);

#[derive(Debug, Clone, Copy, Add, Sub, Neg)]
pub struct AxisOrientedCellCoords {
    x: i32,
    y: i32,
    z: i32,
}

impl AxisOrientedCellCoords {
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
    pub fn from_cell_coords(cell_coords: CellCoords) -> Self {
        let pos = cell_coords.as_ivec3() * CELLS_POSITION_AXIS_ORIENTATION;
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

impl From<IVec3> for AxisOrientedCellCoords {
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
    fn test_cell_coords_new() {
        let cell_coords = CellCoords::new(1, 2, 3);
        assert_eq!(cell_coords.x, 1);
        assert_eq!(cell_coords.y, 2);
        assert_eq!(cell_coords.z, 3);
    }

    #[test]
    fn test_cell_coords_from_ivec3() {
        let ivec3 = IVec3::new(4, 5, 6);
        let cell_coords = CellCoords::from_ivec3(ivec3);
        assert_eq!(cell_coords.x, 4);
        assert_eq!(cell_coords.y, 5);
        assert_eq!(cell_coords.z, 6);
    }

    #[test]
    fn test_cell_coords_as_vec3() {
        let cell_coords = CellCoords::new(1, 2, 3);
        let vec3 = cell_coords.as_vec3();
        assert_eq!(vec3, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_cell_coords_as_ivec3() {
        let cell_coords = CellCoords::new(1, 2, 3);
        let ivec3 = cell_coords.as_ivec3();
        assert_eq!(ivec3, IVec3::new(1, 2, 3));
    }

    #[test]
    fn test_cell_coords_as_game_coordinates_transform() {
        let cell_coords = CellCoords::new(1, 2, 3);
        let transform = cell_coords.as_game_coordinates_transform();
        let expected_transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0))
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2));
        assert_eq!(transform, expected_transform);
    }

    #[test]
    fn test_axis_oriented_cell_coords_from_cell_coords() {
        let cell_coords = CellCoords::new(1, 2, 3);
        let axis_oriented_cell_coords = AxisOrientedCellCoords::from_cell_coords(cell_coords);
        assert_eq!(axis_oriented_cell_coords.x, 1);
        assert_eq!(axis_oriented_cell_coords.y, -3);
        assert_eq!(axis_oriented_cell_coords.z, -2);
    }

    #[test]
    fn test_axis_oriented_cell_coords_as_vec3() {
        let axis_oriented_cell_coords = AxisOrientedCellCoords::new(1, 2, 3);
        let vec3 = axis_oriented_cell_coords.as_vec3();
        assert_eq!(vec3, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_axis_oriented_cell_coords_as_cell_centered_visual_coordinates() {
        assert_eq!(
            AxisOrientedCellCoords::ZERO.as_cell_centered_visual_coordinates(),
            Vec3::new(0.5, -0.5, 0.5)
        );
        assert_eq!(
            AxisOrientedCellCoords::new(1, 2, 3).as_cell_centered_visual_coordinates(),
            Vec3::new(1.5, 1.5, 3.5)
        );
    }
}
