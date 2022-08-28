// use std::ops::{
//     Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
// };

// use super::{vector3::Vector3, vector2::Vector2};

// #[derive(Debug,Clone, Copy)]
// pub struct Vector4<T>
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
//        + From<u32> + From<f32>
// {
//     pub x: T,
//     pub y: T,
//     pub z: T,
//     pub w: T,
// }

// // Index
// impl<T> Index<usize> for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     type Output = T;

//     fn index(&self, index: usize) -> &Self::Output {
//         match index {
//             0 => &self.x,
//             1 => &self.y,
//             2 => &self.z,
//             3 => &self.w,
//             _ => panic!("Index out of bounds"),
//         }
//     }
// }

// // IndexMut
// impl<T> IndexMut<usize> for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         match index {
//             0 => &mut self.x,
//             1 => &mut self.y,
//             2 => &mut self.z,
//             3 => &mut self.w,
//             _ => panic!("Index out of bounds"),
//         }
//     }
// }

// // Add
// impl<T> Add<Vector4<T>> for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     type Output = Vector4<T>;

//     fn add(self, other: Vector4<T>) -> Self::Output {
//         Vector4 {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//             w: self.w + other.w,
//         }
//     }
// }

// //AddAssign
// impl<T> AddAssign<Vector4<T>> for Vector4<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + AddAssign
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32> + From<f32>
// {
//     fn add_assign(&mut self, other: Vector4<T>) {
//         self.x += other.x;
//         self.y += other.y;
//         self.z += other.z;
//         self.w += other.w;
//     }
// }

// // Sub
// impl<T> Sub<Vector4<T>> for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     type Output = Vector4<T>;

//     fn sub(self, other: Vector4<T>) -> Self::Output {
//         Vector4 {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.z - other.z,
//             w: self.w - other.w,
//         }
//     }
// }

// //SubAssign
// impl<T> SubAssign<Vector4<T>> for Vector4<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + SubAssign
//         + PartialEq
//         + From<u32> + From<f32>
// {
//     fn sub_assign(&mut self, other: Vector4<T>) {
//         self.x -= other.x;
//         self.y -= other.y;
//         self.z -= other.z;
//         self.w -= other.w;
//     }
// }

// // Mul
// impl<T> Mul<Vector4<T>> for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     type Output = Vector4<T>;

//     fn mul(self, other: Vector4<T>) -> Self::Output {
//         Vector4 {
//             x: self.x * other.x,
//             y: self.y * other.y,
//             z: self.z * other.z,
//             w: self.w * other.w,
//         }
//     }
// }

// // MulAssign
// impl<T> MulAssign<Vector4<T>> for Vector4<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + MulAssign
//         + PartialEq
//         + From<u32> + From<f32>
// {
//     fn mul_assign(&mut self, other: Vector4<T>) {
//         self.x *= other.x;
//         self.y *= other.y;
//         self.z *= other.z;
//         self.w *= other.w;
//     }
// }

// // Div
// impl<T> Div<Vector4<T>> for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     type Output = Vector4<T>;

//     fn div(self, other: Vector4<T>) -> Self::Output {
//         Vector4 {
//             x: self.x / other.x,
//             y: self.y / other.y,
//             z: self.z / other.z,
//             w: self.w / other.w,
//         }
//     }
// }

// // DivAssign
// impl<T> DivAssign<Vector4<T>> for Vector4<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + DivAssign
//         + PartialEq
//         + From<u32> + From<f32>
// {
//     fn div_assign(&mut self, other: Vector4<T>) {
//         self.x /= other.x;
//         self.y /= other.y;
//         self.z /= other.z;
//         self.w /= other.w;
//     }
// }

// // Neg
// impl<T> Neg for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     type Output = Vector4<T>;

//     fn neg(self) -> Self::Output {
//         Vector4 {
//             x: -self.x,
//             y: -self.y,
//             z: -self.z,
//             w: -self.w,
//         }
//     }
// }

// // Mul scalar
// impl<T> Mul<T> for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     type Output = Vector4<T>;

//     fn mul(self, other: T) -> Self::Output {
//         Vector4 {
//             x: self.x * other,
//             y: self.y * other,
//             z: self.z * other,
//             w: self.w * other,
//         }
//     }
// }

// // MulAssign scalar
// impl<T> MulAssign<T> for Vector4<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + MulAssign
//         + PartialEq
//         + From<u32> + From<f32>
// {
//     fn mul_assign(&mut self, other: T) {
//         self.x *= other;
//         self.y *= other;
//         self.z *= other;
//         self.w *= other;
//     }
// }

// // Div scalar
// impl<T> Div<T> for Vector4<T>
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
//         + From<u32> + From<f32>
// {
//     type Output = Vector4<T>;

//     fn div(self, other: T) -> Self::Output {
//         Vector4 {
//             x: self.x / other,
//             y: self.y / other,
//             z: self.z / other,
//             w: self.w / other,
//         }
//     }
// }

// // DivAssign scalar
// impl<T> DivAssign<T> for Vector4<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + DivAssign
//         + PartialEq
//         + From<u32> + From<f32>
// {
//     fn div_assign(&mut self, other: T) {
//         self.x /= other;
//         self.y /= other;
//         self.z /= other;
//         self.w /= other;
//     }
// }

// impl<T> Vector4<T> where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//          + From<u32> + From<f32>
// {
//     /// Returns the length of the vector.
//     /// # Examples
//     /// ```
//     /// use vek::vec::Vector4;
//     /// let vec = Vector4::new(1.0, 2.0, 3.0, 4.0);
//     /// assert_eq!(vec.length(), 5.47722557);
//     /// ```
//     /// # See also
//     /// [`length_squared`](#method.length_squared)
//     /// [`normalize`](#method.normalize)
//     /// [`normalized`](#method.normalized)
//     /// [`normalized_mut`](#method.normalized_mut)
//     /// [`normalize_mut`](#method.normalize_mut)
//     /// [`normalize_to`](#method.normalize_to)
//     /// [`normalize_to_mut`](#method.normalize_to_mut)
//     /// ```
    
//     pub fn new(x:T,y:T,z:T,w:T)->Self{
//         Vector4 {
//             x,
//             y,
//             z,
//             w,
//         }
//     }
    
//     // xyz
//     pub fn xyz(&self) -> Vector3<T> {
//         Vector3 {
//             x: self.x,
//             y: self.y,
//             z: self.z,
//         }
//     }
    
//     // xy
//     pub fn xy(&self) -> Vector2<T> {
//         Vector2 {
//             x: self.x,
//             y: self.y,
//         }
//     }

//     // rgb
//     pub fn rgb(&self) -> Vector3<T> {
//         Vector3 {
//             x: self.x,
//             y: self.y,
//             z: self.z,
//         }
//     }

//     // rg
//     pub fn rg(&self) -> Vector2<T> {
//         Vector2 {
//             x: self.x, 
//             y: self.y,
//         }
//     }
// }
