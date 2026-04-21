//! This module defines the [Vec3] struct, which represents a mathematical three-
//! dimensional vector.
//!
//! See the C++ implementation of the Vec3 class for comparison:
//! https://github.com/Anatoly03/MolSim-WS25-GroupA/blob/assignment5-local-backup/src/core/math/Vec3.h

use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// generic vec3 method
impl<T> Vec3<T> {
    /// Creates a new [Vec3] instance with the given x, y, and z components.
    ///
    /// # Example
    ///
    /// ```rust
    /// let v: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    /// let w: Vec3<i32> = Vec3::new(1, 2, 3);
    ///
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 2.0);
    /// assert_eq!(v.z, 3.0);
    /// assert_eq!(w.x, 1);
    /// assert_eq!(w.y, 2);
    /// assert_eq!(w.z, 3);
    /// ```
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Creates a new [Vec3] instance with all components set to zero. Requires that
    /// the generic type implements the [Default] trait.
    pub fn zero() -> Self
    where
        T: Default,
    {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

// vector negation
impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;

    /// Implements the unary negation operator `-` for the [Vec3] class.
    ///
    /// # Example
    ///
    /// ```rust
    /// let v: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    /// let w: Vec3<i32> = Vec3::new(1, 2, 3);
    ///
    /// assert_eq!(-v, Vec3::new(-1.0, -2.0, -3.0));
    /// assert_eq!(-w, Vec3::new(-1, -2, -3));
    /// ```
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// vector addition
impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;

    /// Implements the addition operator `+` for the [Vec3] class.
    ///
    /// # Example
    ///
    /// ```rust
    /// let v: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    /// let w: Vec3<f64> = Vec3::new(4.0, 5.0, 6.0);
    ///
    /// assert_eq!(v + w, Vec3::new(5.0, 7.0, 9.0));
    /// ```
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// vector subtraction
impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;

    /// Implements the subtraction operator `-` for the [Vec3] class.
    ///
    /// # Example
    ///
    /// ```rust
    /// let v: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    /// let w: Vec3<f64> = Vec3::new(4.0, 5.0, 6.0);
    ///
    /// assert_eq!(v - w, Vec3::new(-3.0, -3.0, -3.0));
    /// ```
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// scalar multiplication with scalar
impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    /// Implements the scalar multiplication operator `*` for the [Vec3] class.
    ///
    /// # Example
    ///
    /// ```rust
    /// let v: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    /// let w: Vec3<f64> = Vec3::new(2.0, 4.0, 6.0);
    ///
    /// assert_eq!(v * 2.0, w);
    /// ```
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// vector division by scalar
impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    /// Implements the division operator `/` for the [Vec3] class, allowing division of a vector
    /// by a scalar.
    ///
    /// # Example
    /// 
    //// ```rust
    /// let v: Vec3<f64> = Vec3::new(2.0, 4.0, 6.0);
    /// let w: Vec3<f64> = Vec3::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(v / 2.0, w);
    /// ```
    fn div(self, scalar: T) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

macro_rules! impl_for_primitives {
    ($($t:ty),*) => {
$(impl Vec3<$t> {
    /// Creates the dot product of two [Vec3] instances of the same primitive type.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// let w = Vec3::new(4.0, 5.0, 6.0);
    /// 
    /// // TODO think about example output
    /// ```
    pub fn dot(&self, other: &Self) -> $t {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Creates the cross product of two [Vec3] instances of the same primitive type.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// let w = Vec3::new(4.0, 5.0, 6.0);
    /// 
    /// // TODO think about example output
    /// ```
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    /// Computes the length (magnitude) of the [Vec3] instance.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let v = Vec3::new(2.0, 2.0, 1.0);
    /// 
    /// assert_eq!(v.length(), 3.0);
    /// ```
    pub fn length(&self) -> f64 {
        // https://stackoverflow.com/a/29864160
        (self.dot(self) as f64).sqrt()
    }

    /// TODO: rethink: old codebase became buggy because we used the wrong method.
    pub fn length2(&self) -> $t {
        // https://stackoverflow.com/a/29864160
        self.dot(self)
    }

    /// Normalizes the [Vec3] instance to have a length of 1,
    /// returning an option containing a new [Vec3] instance. If
    /// the original vector is zero-length, returns None to avoid
    /// division by zero.
    /// 
    /// **Note: I've made this decision with the intention of
    /// propagating divisions by zero upwards the stack. I do
    /// not know if this will turn out to be useful.**
    pub fn normal(&self) -> Option<Self> {
        match self.length() {
            0.0 => None,
            len => Some(Self {
                x: self.x / len as $t,
                y: self.y / len as $t,
                z: self.z / len as $t,
            }),
        }
    }
})*
    };
}

impl_for_primitives!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);
