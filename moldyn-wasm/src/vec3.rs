//! The module which wraps the core library [Vec3] struct into a WebAssembly
//! compatible binding.

use moldyn_core::Vec3;
use wasm_bindgen::prelude::*;

/// A three-dimensional vector.
#[wasm_bindgen(inspectable, js_name = Vec)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3Wrapper {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[wasm_bindgen]
impl Vec3Wrapper {
    /// Creates a new vector with all components specified
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    /// const vec = new Vec(1, 2, 3);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Creates a new vector with all components set to zero.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    /// const zeroVec = new Vec();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new_zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Adds two vectors together and returns the result as a new vector.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    ///
    /// const vec1 = new Vec(1, 2, 3);
    /// const vec2 = new Vec(2, 3, 4);
    /// const result = vec1.add(vec2); // 3, 5, 7
    /// ```
    #[wasm_bindgen]
    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// Subtracts two vectors and returns the result as a new vector.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    ///
    /// const vec1 = new Vec(2, 3, 4);
    /// const vec2 = new Vec(1, 2, 3);
    /// const result = vec1.sub(vec2); // 1, 1, 1
    /// ```
    #[wasm_bindgen]
    pub fn sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Negates the and returns the result as a new vector.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    ///
    /// const vec = new Vec(2, 3, 4);
    /// const result = vec.neg(); // -2, -3, -4
    /// ```
    #[wasm_bindgen]
    pub fn neg(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    /// Multiplies a vector by a scalar and returns the result as a
    /// new vector.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    ///
    /// const vec = new Vec(2, 3, 4);
    /// const result = vec.mul(2); // 4, 6, 8
    /// ```
    #[wasm_bindgen]
    pub fn mul(&self, scalar: f64) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    /// Divides a vector by a scalar and returns the result as a
    /// new vector.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    ///
    /// const vec = new Vec(2, 4, 6);
    /// const result = vec.div(2); // 1, 2, 3
    /// ```
    #[wasm_bindgen]
    pub fn div(&self, scalar: f64) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }

    /// Creates the dot product of two vector instances and returns the
    /// result as a new vector.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    ///
    /// const vec1 = new Vec(1, 2, 3);
    /// const vec2 = new Vec(4, 5, 6);
    /// const result = vec1.sub(vec2); // 1*4 + 2*5 + 3*6
    /// ```
    #[wasm_bindgen]
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Creates the docrosst product of two vector instances and returns
    /// the result as a new vector.
    ///
    /// # Example
    ///
    /// ```js
    /// import { Vec } from "moldyn-wasm";
    ///
    /// const vec1 = new Vec(1, 2, 3);
    /// const vec2 = new Vec(4, 5, 6);
    /// ```
    #[wasm_bindgen]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Computes the length of the vector and returns it as a scalar
    /// value.
    #[wasm_bindgen]
    pub fn length(&self) -> f64 {
        self.dot(&self).sqrt()
    }

    /// Computes the second-norm of the vector (length without the square
    /// root operation applied) and returns it as a scalar value.
    #[wasm_bindgen]
    pub fn length2(&self) -> f64 {
        self.dot(&self)
    }

    /// Creates the normal a vector instance and returns the result as
    /// a new vector.
    #[wasm_bindgen]
    pub fn normal(&self) -> Self {
        self.div(self.length())
    }
}

impl From<Vec3> for Vec3Wrapper {
    fn from(vec: Vec3) -> Self {
        Self::new(vec.x, vec.y, vec.z)
    }
}

impl Into<Vec3> for Vec3Wrapper {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}
