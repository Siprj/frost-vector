use std::ops::{Add, Div, Neg};

use crate::raw::Gpu;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(C, packed)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add<Vector2<T>> for Vector2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// NOTE: This is highly questionable inmplementaion of `Add` trait.
impl<'a, T> Add<T> for &'a Vector2<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector2<T>;
    #[inline]
    fn add(self, other: T) -> Vector2<T> {
        Vector2 {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl<'a, T> Add<&Vector2<T>> for &'a Vector2<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector2<T>;
    #[inline]
    fn add(self, other: &Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Add<Vector3<T>> for Vector3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// NOTE: This is highly questionable inmplementaion of `Add` trait.
impl<'a, T> Add<T> for &'a Vector3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector3<T>;
    #[inline]
    fn add(self, other: T) -> Vector3<T> {
        Vector3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<'a, T> Add<&Vector3<T>> for &'a Vector3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector3<T>;
    #[inline]
    fn add(self, other: &Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Add<Vector4<T>> for Vector4<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<'a, T> Add<&Vector4<T>> for &'a Vector4<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vector4<T>;
    #[inline]
    fn add(self, other: &Vector4<T>) -> Self::Output {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(C, packed)]
pub struct Matrix4x4<T> {
    pub c0r0: T,
    pub c0r1: T,
    pub c0r2: T,
    pub c0r3: T,
    pub c1r0: T,
    pub c1r1: T,
    pub c1r2: T,
    pub c1r3: T,
    pub c2r0: T,
    pub c2r1: T,
    pub c2r2: T,
    pub c2r3: T,
    pub c3r0: T,
    pub c3r1: T,
    pub c3r2: T,
    pub c3r3: T,
}

impl<T: Sized + Gpu> Gpu for Matrix4x4<T> {}

pub trait Zero<T> {
    fn zero() -> T;
}

impl Zero<f32> for f32 {
    #[inline]
    fn zero() -> f32 {
        0_f32
    }
}
pub trait One<T> {
    fn one() -> T;
}

impl One<f32> for f32 {
    #[inline]
    fn one() -> f32 {
        1_f32
    }
}
pub trait Two<T> {
    fn two() -> T;
}

impl Two<f32> for f32 {
    #[inline]
    fn two() -> f32 {
        2_f32
    }
}

#[inline]
pub fn ortho<T>(width: u16, height: u16) -> Matrix4x4<T>
where
    T: Div<Output = T> + Zero<T> + One<T> + Two<T> + From<u16> + Copy + Neg<Output = T>,
{
    Matrix4x4 {
        c0r0: T::two() / T::from(width),
        c1r0: T::zero(),
        c2r0: T::zero(),
        c3r0: -T::one(),

        c0r1: T::zero(),
        c1r1: -T::two() / T::from(height),
        c2r1: T::zero(),
        c3r1: T::one(),

        c0r2: T::zero(),
        c1r2: T::zero(),
        c2r2: T::one(),
        c3r2: T::zero(),

        c0r3: T::zero(),
        c1r3: T::zero(),
        c2r3: T::zero(),
        c3r3: T::one(),
    }
}
