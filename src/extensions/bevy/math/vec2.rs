use crate::math::{HasZero, Scalar, TransformTrait, Transformable, Vector, Vector2D};
use bevy::math::{Affine2, Vec2};

impl Vector<f32, 2> for Vec2 {
    #[inline(always)]
    fn angle_between(&self, other: Self) -> f32 {
        Vec2::angle_to(*self, other)
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
    fn length(&self) -> f32 {
        Vec2::length(*self)
    }

    #[inline(always)]
    fn length_squared(&self) -> f32 {
        Vec2::length_squared(*self)
    }

    #[inline(always)]
    fn dot(&self, other: &Self) -> f32 {
        Vec2::dot(*self, *other)
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

    #[inline(always)]
    fn splat(value: f32) -> Self {
        Vec2::splat(value)
    }

    #[inline(always)]
    fn from_x(x: f32) -> Self {
        Vec2::new(x, 0.0)
    }

    #[inline(always)]
    fn from_xy(x: f32, y: f32) -> Self {
        Vec2::new(x, y)
    }

    /// drop the z coordinate
    #[inline(always)]
    fn from_xyz(x: f32, y: f32, _: f32) -> Self {
        Vec2::new(x, y)
    }

    #[inline(always)]
    fn is_about(&self, other: &Self, epsilon: f32) -> bool {
        self.x.is_about(other.x, epsilon) && self.y.is_about(other.y, epsilon)
    }
}

impl HasZero for Vec2 {
    #[inline(always)]
    fn zero() -> Self {
        Vec2::ZERO
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        *self == Vec2::ZERO
    }
}

impl Vector2D for Vec2 {
    type S = f32;

    #[inline(always)]
    fn new(x: f32, y: f32) -> Self {
        Vec2::new(x, y)
    }

    /// Angle between two vectors.
    fn angle_tri(&self, a: Self, b: Self) -> f32 {
        Vec2::angle_to(a - *self, b - *self)
    }

    fn perp_dot(&self, other: &Self) -> Self::S {
        Vec2::perp_dot(*self, *other)
    }
}

impl TransformTrait<f32, 2> for Affine2 {
    type Vec = Vec2;
    type Rot = f32;

    #[inline(always)]
    fn identity() -> Self {
        Affine2::IDENTITY
    }

    fn from_rotation(angle: f32) -> Self {
        bevy::math::Affine2::from_angle(angle)
    }

    #[inline(always)]
    fn from_rotation_arc(from: Vec2, to: Vec2) -> Self {
        bevy::math::Affine2::from_angle(from.angle_to(to))
    }

    #[inline(always)]
    fn from_translation(v: Vec2) -> Self {
        bevy::math::Affine2::from_translation(v)
    }

    #[inline(always)]
    fn from_scale(v: Vec2) -> Self {
        bevy::math::Affine2::from_scale(v)
    }

    #[inline(always)]
    fn with_scale(&self, scale: Self::Vec) -> Self {
        bevy::math::Affine2::from_scale(scale) * *self
    }

    #[inline(always)]
    fn with_translation(&self, v: Self::Vec) -> Self {
        bevy::math::Affine2::from_translation(v) * *self
    }

    #[inline(always)]
    fn apply(&self, v: Vec2) -> Vec2 {
        bevy::math::Affine2::transform_point2(self, v)
    }

    #[inline(always)]
    fn apply_vec(&self, v: Vec2) -> Vec2 {
        bevy::math::Affine2::transform_vector2(self, v)
    }

    #[inline(always)]
    fn chain(&self, other: &Self) -> Self {
        *self * *other
    }
}

impl Transformable<2> for Vec2 {
    type S = f32;
    type Rot = f32;
    type Trans = Affine2;
    type Vec = Vec2;
    fn transform(&mut self, t: &Self::Trans) -> &mut Self {
        *self = t.apply(*self);
        self
    }

    fn lerp(&mut self, other: &Self, t: Self::S) -> &mut Self {
        *self = bevy::math::Vec2::lerp(*self, *other, t);
        self
    }
}
