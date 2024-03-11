use bevy::math::bounding::BoundingVolume;

use bevy::math::IVec3;

#[derive(Debug, Default)]
pub struct IBounds3 {
    pub min: IVec3,
    pub max: IVec3,
}

impl IBounds3 {
    #[inline]
    pub fn new(center: IVec3, size: IVec3) -> Self {
        debug_assert!(size.x >= 0 && size.y >= 0 && size.z >= 0);
        let half_size = size / 2;
        Self {
            min: center - half_size,
            max: center + (size - half_size),
        }
    }

    #[inline]
    pub fn size(&self) -> IVec3 {
        self.max - self.min
    }
}

impl BoundingVolume for IBounds3 {
    type Position = IVec3;
    type HalfSize = IVec3;

    #[inline]
    fn center(&self) -> Self::Position {
        (self.min + self.max) / 2
    }

    #[inline]
    fn half_size(&self) -> Self::HalfSize {
        self.size() / 2
    }

    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn visible_area(&self) -> f32 {
        let b = self.size();
        (b.x * (b.y + b.z) + b.y * b.z) as f32
    }

    #[inline]
    fn contains(&self, other: &Self) -> bool {
        other.min.x >= self.min.x
            && other.min.y >= self.min.y
            && other.min.z >= self.min.z
            && other.max.x <= self.max.x
            && other.max.y <= self.max.y
            && other.max.z <= self.max.z
    }

    #[inline]
    fn merge(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    #[inline]
    fn grow(&self, amount: Self::HalfSize) -> Self {
        let b = Self {
            min: self.min - amount,
            max: self.max + amount,
        };
        debug_assert!(b.min.x <= b.max.x && b.min.y <= b.max.y && b.min.z <= b.max.z);
        b
    }

    #[inline]
    fn shrink(&self, amount: Self::HalfSize) -> Self {
        let b = Self {
            min: self.min + amount,
            max: self.max - amount,
        };
        debug_assert!(b.min.x <= b.max.x && b.min.y <= b.max.y && b.min.z <= b.max.z);
        b
    }
}
