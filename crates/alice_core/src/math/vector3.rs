// use std::ops::{
//     Add, AddAssign, Div, DivAssign,  Mul, MulAssign, Neg, Sub, SubAssign,
// };

// use super::vector2::Vector2;

// #[derive(Debug,Default, Clone, Copy, PartialEq, PartialOrd)]
// pub struct Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     pub x: T,
//     pub y: T,
//     pub z: T,
// }

// // Addition
// impl<T> Add<Vector3<T>> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn add(self, other: Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//         }
//     }
// }


// // Addition
// impl<T> Add<&Vector3<T>> for &Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn add(self, other: &Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//         }
//     }
// }

// // Addition Assign
// impl<T> AddAssign<Vector3<T>> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + AddAssign
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     fn add_assign(&mut self, other: Vector3<T>) {
//         self.x += other.x;
//         self.y += other.y;
//         self.z += other.z;
//     }
// }

// // Addition scalar
// impl<T> Add<T> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn add(self, other: T) -> Vector3<T> {
//         Vector3 {
//             x: self.x + other,
//             y: self.y + other,
//             z: self.z + other,
//         }
//     }
// }

// // Addition scalar Assign
// impl<T> AddAssign<T> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + AddAssign
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     fn add_assign(&mut self, other: T) {
//         self.x += other;
//         self.y += other;
//         self.z += other;
//     }
// }

// // Subtraction
// impl<T> Sub<Vector3<T>> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn sub(self, other: Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.z - other.z,
//         }
//     }
// }



// // Subtraction
// impl<T> Sub<&Vector3<T>> for &Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn sub(self, other: &Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.z - other.z,
//         }
//     }
// }

// // Subtraction Assign
// impl<T> SubAssign<Vector3<T>> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + SubAssign
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     fn sub_assign(&mut self, other: Vector3<T>) {
//         self.x -= other.x;
//         self.y -= other.y;
//         self.z -= other.z;
//     }
// }

// // Subtraction scalar
// impl<T> Sub<T> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn sub(self, other: T) -> Vector3<T> {
//         Vector3 {
//             x: self.x - other,
//             y: self.y - other,
//             z: self.z - other,
//         }
//     }
// }

// // Subtraction scalar Assign
// impl<T> SubAssign<T> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + SubAssign
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     fn sub_assign(&mut self, other: T) {
//         self.x -= other;
//         self.y -= other;
//         self.z -= other;
//     }
// }

// // Multiplication
// impl<T> Mul<Vector3<T>> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn mul(self, other: Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: self.x * other.x,
//             y: self.y * other.y,
//             z: self.z * other.z,
//         }
//     }
// }

// // Multiplication Assign
// impl<T> MulAssign<Vector3<T>> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + MulAssign
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     fn mul_assign(&mut self, other: Vector3<T>) {
//         self.x *= other.x;
//         self.y *= other.y;
//         self.z *= other.z;
//     }
// }

// // Multiplication scalar
// impl<T> Mul<T> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn mul(self, other: T) -> Vector3<T> {
//         Vector3 {
//             x: self.x * other,
//             y: self.y * other,
//             z: self.z * other,
//         }
//     }
// }

// // Multiplication scalar Assign
// impl<T> MulAssign<T> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + MulAssign
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     fn mul_assign(&mut self, other: T) {
//         self.x *= other;
//         self.y *= other;
//         self.z *= other;
//     }
// }

// // Division
// impl<T> Div<Vector3<T>> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn div(self, other: Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: self.x / other.x,
//             y: self.y / other.y,
//             z: self.z / other.z,
//         }
//     }
// }

// // Division Assign
// impl<T> DivAssign<Vector3<T>> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + DivAssign
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     fn div_assign(&mut self, other: Vector3<T>) {
//         self.x /= other.x;
//         self.y /= other.y;
//         self.z /= other.z;
//     }
// }

// // Division scalar
// impl<T> Div<T> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn div(self, other: T) -> Vector3<T> {
//         Vector3 {
//             x: self.x / other,
//             y: self.y / other,
//             z: self.z / other,
//         }
//     }
// }

// // Division scalar Assign
// impl<T> DivAssign<T> for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + DivAssign
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     fn div_assign(&mut self, other: T) {
//         self.x /= other;
//         self.y /= other;
//         self.z /= other;
//     }
// }

// // Negation
// impl<T> Neg for Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     type Output = Vector3<T>;

//     fn neg(self) -> Vector3<T> {
//         Vector3 {
//             x: -self.x,
//             y: -self.y,
//             z: -self.z,
//         }
//     }
// }

// impl<T> Vector3<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + Into<f32>
//         + From<f32>,
// {
//     /// Create a new Vector3
//     /// # Arguments
//     /// * `x` - The x component of the vector
//     /// * `y` - The y component of the vector
//     /// * `z` - The z component of the vector
//     /// # Example
//     /// ```
//     /// use vector3::Vector3;
//     /// let vec = Vector3::new(1.0, 2.0, 3.0);
//     /// ```
//     /// # Example
//     /// ```
//     /// use vector3::Vector3;
//     /// let vec = Vector3::new(1.0, 2.0, 3.0);
//     /// assert_eq!(vec.x, 1.0);
//     /// assert_eq!(vec.y, 2.0);
//     /// assert_eq!(vec.z, 3.0);
//     /// ```
//     /// # Example
//     /// ```
//     /// use vector3::Vector3;
//     /// let vec = Vector3::new(1.0, 2.0, 3.0);
//     /// assert_eq!(vec.x, 1.0);
//     /// assert_eq!(vec.y, 2.0);
//     /// assert_eq!(vec.z, 3.0);
//     /// ```
//     ///
//     pub fn new(x: T, y: T, z: T) -> Vector3<T> {
//         Vector3 { x, y, z }
//     }

//     pub fn new_from_vec3(vec: Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: vec.x,
//             y: vec.y,
//             z: vec.z,
//         }
//     }

//     pub fn new_from_vec2(vec: Vector2<T>) -> Vector3<T> {
//         Vector3 {
//             x: vec.x,
//             y: vec.y,
//             z: T::default(),
//         }
//     }

//     // dot product
//     pub fn dot(&self, other: &Vector3<T>) -> T {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }

//     // cross product
//     pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: self.y * other.z - self.z * other.y,
//             y: self.z * other.x - self.x * other.z,
//             z: self.x * other.y - self.y * other.x,
//         }
//     }



//     // length
//     pub fn length(&self) -> T {
//         f32::sqrt((self.x * self.x + self.y * self.y + self.z * self.z).into()).into()
//     }

//     // length squared
//     pub fn length_squared(&self) -> T {
//         self.x * self.x + self.y * self.y + self.z * self.z
//     }

//     // normalize
//     pub fn normalize(&self) -> Vector3<T> {
//         let len = self.length();
//         Vector3 {
//             x: self.x / len,
//             y: self.y / len,
//             z: self.z / len,
//         }
//     }

   
// }



