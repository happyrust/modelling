use crate::math::{Vector, Vector2D};
use bevy::math::{Affine2, Vec2, Vec3};

impl Vector<f32> for Vec2 {
    type Vec2D = Vec2;
    type Vec3D = Vec3;
    type Transform = Affine2;

    #[inline(always)]
    fn zero() -> Self {
        Vec2::ZERO
    }

    #[inline(always)]
    fn dimensions() -> usize {
        2
    }

    #[inline(always)]
    fn distance(&self, other: &Self) -> f32 {
        Vec2::distance(*self, *other)
    }

    #[inline(always)]
    fn distance_squared(&self, other: &Self) -> f32 {
        Vec2::distance_squared(*self, *other)
    }

    #[inline(always)]
    fn dot(&self, other: &Self) -> f32 {
        Vec2::dot(*self, *other)
    }

    #[inline(always)]
    fn cross(&self, other: &Self) -> Self {
        Vec2::new(self.x() * other.y() - self.y() * other.x(), 0.0)
    }

    #[inline(always)]
    fn x(&self) -> f32 {
        self.x
    }

    #[inline(always)]
    fn y(&self) -> f32 {
        self.y
    }

    #[inline(always)]
    fn z(&self) -> f32 {
        0.0
    }

    #[inline(always)]
    fn w(&self) -> f32 {
        0.0
    }

    #[inline(always)]
    fn normalize(&self) -> Self {
        Vec2::normalize(*self)
    }
}

impl Vector2D<f32> for Vec2 {
    #[inline(always)]
    fn from_xy(x: f32, y: f32) -> Self {
        Vec2::new(x, y)
    }
}
