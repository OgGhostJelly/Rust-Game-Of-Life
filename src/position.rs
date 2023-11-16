use std::{
    marker::Copy,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use vecmath::{self, traits, Vector2};

/// A wrapper around [`vecmath::Vector2`]. Allows using [`vecmath`] methods using dot notation and ops.
#[derive(Clone, Copy)]
pub struct Position2D<T>(Vector2<T>);

impl<T> Position2D<T> {
    pub fn add(self, rhs: Position2D<T>) -> Position2D<T>
    where
        T: Copy + Add<Output = T>,
    {
        vecmath::vec2_add(self.unwrap(), rhs.unwrap()).to_pos2()
    }

    pub fn cast<U>(self) -> Position2D<U>
    where
        T: Copy + traits::Cast<U>,
    {
        vecmath::vec2_cast(self.unwrap()).to_pos2()
    }

    pub fn cross(self, rhs: Position2D<T>) -> T
    where
        T: Copy + Mul<T, Output = T> + Sub<T, Output = T>,
    {
        vecmath::vec2_cross(self.unwrap(), rhs.unwrap())
    }

    pub fn dot(self, rhs: Position2D<T>) -> T
    where
        T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
    {
        vecmath::vec2_dot(self.unwrap(), rhs.unwrap())
    }

    pub fn inv_len(self) -> T
    where
        T: Copy
            + traits::One
            + traits::Sqrt
            + Add<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        vecmath::vec2_inv_len(self.unwrap())
    }

    pub fn len(self) -> T
    where
        T: Copy + traits::Sqrt + Add<T, Output = T> + Mul<T, Output = T>,
    {
        vecmath::vec2_len(self.unwrap())
    }

    pub fn normalized(self) -> Position2D<T>
    where
        T: Copy
            + traits::One
            + traits::Sqrt
            + Add<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>,
    {
        vecmath::vec2_normalized(self.unwrap()).to_pos2()
    }

    pub fn normalized_sub(self, rhs: Position2D<T>) -> Position2D<T>
    where
        T: Copy
            + traits::One
            + traits::Sqrt
            + Add<T, Output = T>
            + Mul<T, Output = T>
            + Div<T, Output = T>
            + Sub<T, Output = T>,
    {
        vecmath::vec2_normalized_sub(self.unwrap(), rhs.unwrap()).to_pos2()
    }

    pub fn square_len(self) -> T
    where
        T: Copy + Add<T, Output = T> + Mul<T, Output = T>,
    {
        vecmath::vec2_square_len(self.unwrap())
    }

    pub fn sub(self, rhs: Position2D<T>) -> Position2D<T>
    where
        T: Copy + Sub<T, Output = T>,
    {
        vecmath::vec2_sub(self.unwrap(), rhs.unwrap()).to_pos2()
    }

    pub fn scale(self, rhs: T) -> Position2D<T>
    where
        T: Copy + Mul<Output = T>,
    {
        vecmath::vec2_scale(self.unwrap(), rhs).to_pos2()
    }

    pub fn div_scale(self, rhs: T) -> Position2D<T>
    where
        T: Copy + Div<Output = T>,
    {
        let vec = self.unwrap();

        [vec[0] / rhs, vec[1] / rhs].to_pos2()
    }

    pub fn div(self, rhs: Position2D<T>) -> Position2D<T>
    where
        T: Copy + Div<Output = T>,
    {
        [self.0[0] / rhs.0[0], self.0[1] / rhs.0[1]].to_pos2()
    }

    pub fn mul(self, rhs: Position2D<T>) -> Position2D<T>
    where
        T: Copy + Mul<Output = T>,
    {
        vecmath::vec2_mul(self.unwrap(), rhs.unwrap()).to_pos2()
    }

    pub fn neg(self) -> Position2D<T>
    where
        T: Copy + Neg<Output = T>,
    {
        vecmath::vec2_neg(self.unwrap()).to_pos2()
    }

    pub fn x(self) -> T where T: Copy {
        self.0[0]
    }

    pub fn y(self) -> T where T: Copy {
        self.0[1]
    }

    pub fn unwrap(self) -> Vector2<T> {
        self.0
    }
}

// Impl Ops //

// Scale
impl<T: Copy + Mul<Output = T>> Mul<T> for Position2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.scale(rhs)
    }
}

impl<T: Copy + Mul<Output = T>> MulAssign<T> for Position2D<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = self.scale(rhs)
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Position2D<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.div_scale(rhs)
    }
}

impl<T: Copy + Div<Output = T>> DivAssign<T> for Position2D<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = self.div_scale(rhs)
    }
}

// Mul
impl<T: Copy + Mul<Output = T>> Mul<Position2D<T>> for Position2D<T> {
    type Output = Self;

    fn mul(self, rhs: Position2D<T>) -> Self::Output {
        self.mul(rhs)
    }
}

impl<T: Copy + Mul<Output = T>> MulAssign<Position2D<T>> for Position2D<T> {
    fn mul_assign(&mut self, rhs: Position2D<T>) {
        *self = self.mul(rhs)
    }
}

// Div
impl<T: Copy + Div<Output = T>> Div<Position2D<T>> for Position2D<T> {
    type Output = Self;

    fn div(self, rhs: Position2D<T>) -> Self::Output {
        self.div(rhs)
    }
}

impl<T: Copy + Div<Output = T>> DivAssign<Position2D<T>> for Position2D<T> {
    fn div_assign(&mut self, rhs: Position2D<T>) {
        *self = self.div(rhs)
    }
}

// Add
impl<T: Copy + Add<Output = T>> Add<Position2D<T>> for Position2D<T> {
    type Output = Self;

    fn add(self, rhs: Position2D<T>) -> Self::Output {
        self.add(rhs)
    }
}

impl<T: Copy + Add<Output = T>> AddAssign<Position2D<T>> for Position2D<T> {
    fn add_assign(&mut self, rhs: Position2D<T>) {
        *self = self.add(rhs)
    }
}

// Sub
impl<T: Copy + Sub<Output = T>> Sub<Position2D<T>> for Position2D<T> {
    type Output = Self;

    fn sub(self, rhs: Position2D<T>) -> Self::Output {
        self.sub(rhs)
    }
}

impl<T: Copy + Sub<Output = T>> SubAssign<Position2D<T>> for Position2D<T> {
    fn sub_assign(&mut self, rhs: Position2D<T>) {
        *self = self.sub(rhs)
    }
}

// Type Conversion //

impl<T> From<Vector2<T>> for Position2D<T> {
    fn from(value: [T; 2]) -> Self {
        Self(value)
    }
}

impl From<Position2D<f64>> for sdl2::rect::Point {
    fn from(value: Position2D<f64>) -> Self {
        sdl2::rect::Point::new(value.0[0] as i32, value.0[1] as i32)
    }
}

impl From<Position2D<usize>> for Position2D<f64> {
    fn from(value: Position2D<usize>) -> Self {
        [
            value.0[0] as f64,
            value.0[1] as f64,
        ].into()
    }
}

impl From<Position2D<f64>> for Position2D<u32> {
    fn from(value: Position2D<f64>) -> Self {
        [
            value.0[0] as u32,
            value.0[1] as u32,
        ].into()
    }
}

impl From<Position2D<u32>> for Position2D<f64> {
    fn from(value: Position2D<u32>) -> Self {
        [
            value.0[0] as f64,
            value.0[1] as f64,
        ].into()
    }
}

pub trait ToPosition2D<T> {
    fn to_pos2(self) -> Position2D<T>;
}

impl<T, U> ToPosition2D<T> for U
where
    Position2D<T>: From<U>,
{
    fn to_pos2(self) -> Position2D<T> {
        self.into()
    }
}
