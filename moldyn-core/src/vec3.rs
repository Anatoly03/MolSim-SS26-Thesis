//! This module defines the [Vec3] struct, which represents a mathematical three-
//! dimensional vector.
//!
//! See the C++ implementation of the Vec3 class for comparison:
//! https://github.com/Anatoly03/MolSim-WS25-GroupA/blob/assignment5-local-backup/src/core/math/Vec3.h

use serde::{Deserialize, Serialize, de::Visitor};
use std::{
    fmt::Display, marker::PhantomData, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}
};

/// A struct representing a three-dimensional [mathematical vector](https://en.wikipedia.org/wiki/Vector_%28mathematics_and_physics%29).
/// These are used to represent particle information, such as position
/// or velocity, in three-dimensional space.
#[derive(Debug, PartialEq, Clone, Serialize, Default)]
pub struct Vec3<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// consider alternative implementation (overkill for this project):
// pub struct IVec<const N: usize, T = f64> ([T; N]);

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

    /// Maps the individual components of the [Vec3] instance to a new type
    /// with the provided lambda expression.
    pub fn map<U, F>(self, f: F) -> Vec3<U>
    where
        F: Fn(T) -> U,
    {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }
}

//
// THE FOLLOWING ARE TRAIT IMPLEMENTATIONS FOR THE [Vec3] STRUCT
//

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

// vector addition assignment
impl<T: AddAssign> AddAssign for Vec3<T> {
    /// Implements the addition-assign operation `+=` for the [Vec3] class.
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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

// vector subtraction assignment
impl<T: SubAssign> SubAssign for Vec3<T> {
    /// Implements the subtraction-assign operation `-=` for the [Vec3] class.
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// multiplication with scalar
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

// multiplication with scalar assignment
impl<T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    /// Implements the multiplication-assign operation `*=` for the [Vec3] class.
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
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

// division by scalar assignment
impl<T: DivAssign + Copy> DivAssign<T> for Vec3<T> {
    /// Implements the division-assign operation `/=` for the [Vec3] class.
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

//
// THE FOLLOWING IMPLEMENTS DESERIALIZATION FOR THE [Vec3] STRUCT
// FROM A THREE-ELEMENT SEQUENCE AND THE X-Y-Z OBJECT
//

// TODO test (i love my code after hyperfixations, it looks so gloriously clean, but it exhausts my ability to test)
// see serde deserialization: https://serde.rs/impl-deserialize.html

// We need phantom data because of generics.
// see https://doc.rust-lang.org/stable/nomicon/phantom-data.html
struct Vec3Visitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for Vec3Visitor<T>
where
    T: Deserialize<'de> + Default,
{
    type Value = Vec3<T>;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("3-element array [x, y, z] or object {x, y, z}")
    }

    /// We deserialize three-element sequences into [Vec3] instances by mapping
    /// the first element to x, the second to y, and the third to z. Missing values
    /// are treated as zeroes.
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let x = match seq.next_element() {
            // [x, ...]
            Ok(Some(value)) => value,
            // []
            Ok(None) => return Ok(Vec3::default()),
            Err(e) => return Err(e),
        };

        let y = match seq.next_element() {
            // [x, y, ...]
            Ok(Some(value)) => value,
            // [x]
            Ok(None) => {
                return Ok(Vec3 {
                    x,
                    ..Default::default()
                });
            }
            Err(e) => return Err(e),
        };

        let z = match seq.next_element() {
            // [x, y, z, ...]
            Ok(Some(value)) => value,
            // [x, y]
            Ok(None) => {
                return Ok(Vec3 {
                    x,
                    y,
                    ..Default::default()
                });
            }
            Err(e) => return Err(e),
        };

        Ok(Vec3 { x, y, z })
    }

    /// We deserialize maps into [Vec3] instances by looking for keys "x", "y",
    /// and "z" respectively. If any of these keys are missing, default to zero.
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;
        let mut z = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "x" => x = Some(map.next_value()?),
                "y" => y = Some(map.next_value()?),
                "z" => z = Some(map.next_value()?),
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        // default to zero
        let x = x.unwrap_or(T::default());
        let y = y.unwrap_or(T::default());
        let z = z.unwrap_or(T::default());

        Ok(Vec3 { x, y, z })
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec3<T>
where
    T: Deserialize<'de> + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(Vec3Visitor(PhantomData))
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

macro_rules! impl_for_primitives {
    ($($t:ty),*) => {$(
impl Vec3<$t> {
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
        // divide by zero actually panics: https://internals.rust-lang.org/t/question-why-does-dividing-by-zero-have-no-safety-guarantees/19189
        match self.length() {
            0.0 => None,
            len => Some(Self {
                x: self.x / len as $t,
                y: self.y / len as $t,
                z: self.z / len as $t,
            }),
        }
    }
}

impl Copy for Vec3<$t> {}
    )*};
}

impl_for_primitives!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
