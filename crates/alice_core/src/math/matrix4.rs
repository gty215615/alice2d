
// use super::{vector3::Vector3, vector4::Vector4};
// use std::{ops::{
//     Add,  Div,  Index, IndexMut, Mul, Neg, Sub,
// }, fmt::Debug};

// #[derive(Debug,Clone,Copy,Default,PartialEq,Eq)]
// pub struct Matrix4<T>
// where
//     T: Copy
//         + Default
//         + PartialOrd
//         + Clone
//         + Copy
//         + Add<Output = T>
//         + Sub<Output = T>
//         + Mul<Output = T>
//         + Div<Output = T>
//         + Neg<Output = T>
//         + PartialEq
//         + From<u32>
//         + From<f32>,
// {
//     pub m00: T, pub m10: T, pub m20: T, pub m30: T,
//     pub m01: T, pub m11: T, pub m21: T, pub m31: T,
//     pub m02: T, pub m12: T, pub m22: T, pub m32: T,
//     pub m03: T, pub m13: T, pub m23: T, pub m33: T,
// }


// // multiplication mat4 * mat4
// impl<T> Mul<&Matrix4<T>> for &Matrix4<T>
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
//     type Output = Matrix4<T>;

//     fn mul(self, rhs: &Matrix4<T>) -> Matrix4<T> {
//         Matrix4 {
//             m00: self.m00 * rhs.m00 + self.m01 * rhs.m10 + self.m02 * rhs.m20 + self.m03 * rhs.m30,
//             m01: self.m00 * rhs.m01 + self.m01 * rhs.m11 + self.m02 * rhs.m21 + self.m03 * rhs.m31,
//             m02: self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02 * rhs.m22 + self.m03 * rhs.m32,
//             m03: self.m00 * rhs.m03 + self.m01 * rhs.m13 + self.m02 * rhs.m23 + self.m03 * rhs.m33,
//             m10: self.m10 * rhs.m00 + self.m11 * rhs.m10 + self.m12 * rhs.m20 + self.m13 * rhs.m30,
//             m11: self.m10 * rhs.m01 + self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
//             m12: self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
//             m13: self.m10 * rhs.m03 + self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
//             m20: self.m20 * rhs.m00 + self.m21 * rhs.m10 + self.m22 * rhs.m20 + self.m23 * rhs.m30,
//             m21: self.m20 * rhs.m01 + self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
//             m22: self.m20 * rhs.m02 + self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
//             m23: self.m20 * rhs.m03 + self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
//             m30: self.m30 * rhs.m00 + self.m31 * rhs.m10 + self.m32 * rhs.m20 + self.m33 * rhs.m30,
//             m31: self.m30 * rhs.m01 + self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
//             m32: self.m30 * rhs.m02 + self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
//             m33: self.m30 * rhs.m03 + self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
//         }
//     }   
// }

// // multiplication mat4 * vec4
// impl<T> Mul<Vector4<T>> for Matrix4<T>
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
//     type Output = Vector4<T>;

//     fn mul(self, rhs: Vector4<T>) -> Vector4<T> {
//         Vector4 {
//             x: self.m00 * rhs.x + self.m01 * rhs.y + self.m02 * rhs.z + self.m03 * rhs.w,
//             y: self.m10 * rhs.x + self.m11 * rhs.y + self.m12 * rhs.z + self.m13 * rhs.w,
//             z: self.m20 * rhs.x + self.m21 * rhs.y + self.m22 * rhs.z + self.m23 * rhs.w,
//             w: self.m30 * rhs.x + self.m31 * rhs.y + self.m32 * rhs.z + self.m33 * rhs.w,
//         }
//     }
// }

// // multiplication mat4 * vec3
// impl<T> Mul<Vector3<T>> for Matrix4<T>
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

//     fn mul(self, rhs: Vector3<T>) -> Vector3<T> {
//         Vector3 {
//             x: self.m00 * rhs.x + self.m10 * rhs.y + self.m20 * rhs.z,
//             y: self.m01 * rhs.x + self.m11 * rhs.y + self.m21 * rhs.z,
//             z: self.m02 * rhs.x + self.m12 * rhs.y + self.m22 * rhs.z,
//         }
//     }
// }


// // Index matrix4
// impl<T> Index<usize> for Matrix4<T>
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
//     type Output = T;

//     fn index(&self, index: usize) -> &T {
//         match index {
//             0 => &self.m00,
//             1 => &self.m10,
//             2 => &self.m20,
//             3 => &self.m30,
//             4 => &self.m01,
//             5 => &self.m11,
//             6 => &self.m21,
//             7 => &self.m31,
//             8 => &self.m02,
//             9 => &self.m12,
//             10 => &self.m22,
//             11 => &self.m32,
//             12 => &self.m03,
//             13 => &self.m13,
//             14 => &self.m23,
//             15 => &self.m33,
//             _ => panic!("Index out of bounds"),
//         }
//     }
// }


// // Index mut matrix4
// impl<T> IndexMut<usize> for Matrix4<T>
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
//     fn index_mut(&mut self, index: usize) -> &mut T {
//         match index {
//             0 => &mut self.m00,
//             1 => &mut self.m10,
//             2 => &mut self.m20,
//             3 => &mut self.m30,
//             4 => &mut self.m01,
//             5 => &mut self.m11,
//             6 => &mut self.m21,
//             7 => &mut self.m31,
//             8 => &mut self.m02,
//             9 => &mut self.m12,
//             10 => &mut self.m22,
//             11 => &mut self.m32,
//             12 => &mut self.m03,
//             13 => &mut self.m13,
//             14 => &mut self.m23,
//             15 => &mut self.m33,
//             _ => panic!("Index out of bounds"),
//         }
//     }
// }














// impl<T> Matrix4<T>
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
//         + From<f32>
//         + Into<f32>
//         + Debug,
// {
//     pub fn new(
//         m00: T,
//         m01: T,
//         m02: T,
//         m03: T,
//         m10: T,
//         m11: T,
//         m12: T,
//         m13: T,
//         m20: T,
//         m21: T,
//         m22: T,
//         m23: T,
//         m30: T,
//         m31: T,
//         m32: T,
//         m33: T,
//     ) -> Matrix4<T> {
//         Matrix4 {
//             m00,
//             m01,
//             m02,
//             m03,
//             m10,
//             m11,
//             m12,
//             m13,
//             m20,
//             m21,
//             m22,
//             m23,
//             m30,
//             m31,
//             m32,
//             m33,
//         }
//     }
//     pub fn webgl_to_wgpu(mat:&Matrix4<T>)->Matrix4<T>
    
//     {
//         &Matrix4::new(
//             (1.0).into(), T::default(), T::default(), T::default(),
//             T::default(), (1.0).into(), T::default(), T::default(),
//             T::default(), T::default(), (0.5).into(), (0.5).into(),
//             T::default(), T::default(), T::default(), (1.0).into(),
//             ) * mat
//     }
//     // zero matrix
//     pub fn zero() -> Matrix4<T> {
//         Matrix4::new(
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//         )
//     }

//     // identity matrix
//     pub fn identity() -> Matrix4<T> {
//         Matrix4::new(
//             T::from(1.0),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::from(1.0),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::from(1.0),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::from(1.0),
//         )
//     }

//     // translation matrix
//     pub fn new_translation(v: &Vector3<T>) -> Matrix4<T> {
//         Matrix4::new(
//             T::from(1.0),
//             T::default(),
//             T::default(),
//             v.x,
//             T::default(),
//             T::from(1.0),
//             T::default(),
//             v.y,
//             T::default(),
//             T::default(),
//             T::from(1.0),
//             v.z,
//             T::default(),
//             T::default(),
//             T::default(),
//             T::from(1.0),
//         )
//     }

//     // translation matrix
//     pub fn translation(mut mat4: Matrix4<T>, v: &Vector3<T>) -> Matrix4<T> {
//         mat4.m03 = v.x;
//         mat4.m13 = v.y;
//         mat4.m23 = v.z;
//         mat4
//     }

//     // rotation matrix
//     pub fn new_rotation(v: &Vector3<T>) -> Matrix4<T> {
//         let x = (v.x).into().to_radians();
//         let y = v.y.into().to_radians();
//         let z = v.z.into().to_radians();

//         let sx = x.sin();
//         let cx = x.cos();
//         let sy = y.sin();
//         let cy = y.cos();
//         let sz = z.sin();
//         let cz = z.cos();

//         Matrix4::new(
//             (cy * cz).into(),
//             (-cy * sz).into(),
//             sy.into(),
//             T::default(),
//             (cx * sz + sx * sy * cz).into(),
//             (cx * cz - sx * sy * sz).into(),
//             (-sx * cy).into(),
//             T::default(),
//             (sx * sz - cx * sy * cz).into(),
//             (sx * cz + cx * sy * sz).into(),
//             (cx * cy).into(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::from(1.0),
//         )
//     }

//     // rotation matrix
//     pub fn rotation(mut mat4: Matrix4<T>, v: &Vector3<T>) -> Matrix4<T> {
//         let x = v.x.into().to_radians();
//         let y = v.y.into().to_radians();
//         let z = v.z.into().to_radians();

//         let sx = x.sin();
//         let cx = x.cos();
//         let sy = y.sin();
//         let cy = y.cos();
//         let sz = z.sin();
//         let cz = z.cos();

//         mat4.m00 = (cy * cz).into();
//         mat4.m01 = (-cy * sz).into();
//         mat4.m02 = sy.into();
//         mat4.m10 = (cx * sz + sx * sy * cz).into();
//         mat4.m11 = (cx * cz - sx * sy * sz).into();
//         mat4.m12 = (-sx * cy).into();
//         mat4.m20 = (sx * sz - cx * sy * cz).into();
//         mat4.m21 = (sx * cz + cx * sy * sz).into();
//         mat4.m22 = (cx * cy).into();
//         mat4.m33 = T::from(1.0);
//         mat4
//     }

//     // scale matrix
//     pub fn new_scale(v: &Vector3<T>) -> Matrix4<T> {
//         Matrix4::new(
//             v.x,
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             v.y,
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             v.z,
//             T::default(),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::from(1.0),
//         )
//     }

//     // scale matrix
//     pub fn scale(mut mat4: Matrix4<T>, v: &Vector3<T>) -> Matrix4<T> {
//         mat4.m00 = v.x;
//         mat4.m11 = v.y;
//         mat4.m22 = v.z;
//         mat4
//     }

//     // perspective matrix
//     pub fn perspective(fov: T, aspect: T, near: T, far: T) -> Matrix4<T> {


//         let f: T = (1.0_f32 / (fov.into() / 2.0).tan()).into();
//         Matrix4::new(
//             f / aspect,     T::default(),       T::default(),T::default(),
//             T::default(),   f,T::default(), T::default(),
//             T::default(),   T::default(),       (far + near) / (near - far), (T::from(2.0) * far * near) / (near - far),
//             T::default(),   T::default(),       T::from(-1.0),T::default(),
//         )
        
//     }

//     // orthographic matrix
//     pub fn orthographic(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Matrix4<T> {
//         Matrix4::new(
//             T::from(2.0) / (right - left),
//             T::default(),
//             T::default(),
//             -(right + left) / (right - left),
//             T::default(),
//             T::from(2.0) / (top - bottom),
//             T::default(),
//             -(top + bottom) / (top - bottom),
//             T::default(),
//             T::default(),
//             T::from(2.0) / (near - far),
//             -(near + far) / (near - far),
//             T::default(),
//             T::default(),
//             T::default(),
//             T::from(1.0),
//         )
//     }

//     // look at matrix
//     pub fn look_at(eye: Vector3<T>, center: Vector3<T>, up: Vector3<T>) -> Matrix4<T> {
//         let f = (&center - &eye).normalize();
//         let s = f.cross(&up).normalize();
//         let u = s.cross(&f);

//         Matrix4::new(
//             s.x,
//             u.x,
//             -f.x,
//             T::default(),
//             s.y,
//             u.y,
//             -f.y,
//             T::default(),
//             s.z,
//             u.z,
//             -f.z,
//             T::default(),
//             -s.dot(&eye),
//             -u.dot(&eye),
//             f.dot(&eye),
//             T::from(1.0),
//         )
//     }

//     // transpose matrix
//     pub fn transpose(&self) -> Matrix4<T> {
//         Matrix4::new(
//             self.m00, self.m10, self.m20, self.m30, self.m01, self.m11, self.m21, self.m31,
//             self.m02, self.m12, self.m22, self.m32, self.m03, self.m13, self.m23, self.m33,
//         )
//     }

//     // determinant
//     pub fn determinant(&self) -> T {
//         let a = self.m00 * self.m11 - self.m01 * self.m10;
//         let b = self.m00 * self.m12 - self.m02 * self.m10;
//         let c = self.m00 * self.m13 - self.m03 * self.m10;
//         let d = self.m01 * self.m12 - self.m02 * self.m11;
//         let e = self.m01 * self.m13 - self.m03 * self.m11;
//         let f = self.m02 * self.m13 - self.m03 * self.m12;
//         let g = self.m20 * self.m31 - self.m21 * self.m30;
//         let h = self.m20 * self.m32 - self.m22 * self.m30;
//         let i = self.m20 * self.m33 - self.m23 * self.m30;
//         let j = self.m21 * self.m32 - self.m22 * self.m31;
//         let k = self.m21 * self.m33 - self.m23 * self.m31;
//         let l = self.m22 * self.m33 - self.m23 * self.m32;

//         a * l - b * k + c * j + d * i - e * h + f * g
//     }
  
//     // inverse matrix
//     pub fn inverse(&self) -> Matrix4<T> {
//         let mut mat4 = Matrix4::identity();
   
//         let b00 = self.m00 * self.m11 - self.m10 * self.m01;
//         let b01 = self.m00 * self.m21 - self.m20 * self.m01;
//         let b02 = self.m00 * self.m31 - self.m30 * self.m01;
//         let b03 = self.m10 * self.m21 - self.m20 * self.m11;
//         let b04 = self.m10 * self.m31 - self.m30 * self.m11;
//         let b05 = self.m20 * self.m31 - self.m30 * self.m21;
//         let b06 = self.m02 * self.m13 - self.m12 * self.m03;
//         let b07 = self.m02 * self.m23 - self.m22 * self.m03;
//         let b08 = self.m02 * self.m33 - self.m32 * self.m03;
//         let b09 = self.m12 * self.m23 - self.m22 * self.m13;
//         let b10 = self.m12 * self.m33 - self.m32 * self.m13;
//         let b11 = self.m22 * self.m33 - self.m32 * self.m23; // Calculate the determinant

//         let  det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

//         if det == T::default() {
//             return mat4;
//         }
//         let invdet = T::from(1.0) / det;
      
//         mat4.m00 =  (self.m11 * b11 - self.m21 * b10 + self.m31 * b09) * invdet;
//         mat4.m10 =  (self.m20 * b10 - self.m10 * b11 - self.m30 * b09) * invdet;
//         mat4.m20 =  (self.m13 * b05 - self.m23 * b04 + self.m33 * b03) * invdet;
//         mat4.m30 =  (self.m22 * b04 - self.m12 * b05 - self.m32 * b03) * invdet;
//         mat4.m01 =  (self.m21 * b08 - self.m01 * b11 - self.m31 * b07) * invdet;
//         mat4.m11 =  (self.m00 * b11 - self.m20 * b08 + self.m30 * b07) * invdet;
//         mat4.m21 =  (self.m23 * b02 - self.m03 * b05 - self.m33 * b01) * invdet;
//         mat4.m31 =  (self.m02 * b05 - self.m22 * b02 + self.m32 * b01) * invdet;
//         mat4.m02 =  (self.m01 * b10 - self.m11 * b08 + self.m31 * b06) * invdet;
//         mat4.m12 =  (self.m10 * b08 - self.m00 * b10 - self.m30 * b06) * invdet;
//         mat4.m22 = (self.m03 * b04 - self.m13 * b02 + self.m33 * b00) * invdet;
//         mat4.m32 = (self.m12 * b02 - self.m02 * b04 - self.m32 * b00) * invdet;
//         mat4.m03 = (self.m11 * b07 - self.m01 * b09 - self.m21 * b06) * invdet;
//         mat4.m13 = (self.m00 * b09 - self.m10 * b07 + self.m20 * b06) * invdet;
//         mat4.m23 = (self.m13 * b01 - self.m03 * b03 - self.m23 * b00) * invdet;
//         mat4.m33 = (self.m02 * b03 - self.m12 * b01 + self.m22 * b00) * invdet;

//         mat4
//     }
















//     // multiply matrix
//     pub fn mul(&self, other: &Matrix4<T>) -> Matrix4<T> {
//         Matrix4::new(
//             self.m00 * other.m00
//                 + self.m01 * other.m10
//                 + self.m02 * other.m20
//                 + self.m03 * other.m30,
//             self.m00 * other.m01
//                 + self.m01 * other.m11
//                 + self.m02 * other.m21
//                 + self.m03 * other.m31,
//             self.m00 * other.m02
//                 + self.m01 * other.m12
//                 + self.m02 * other.m22
//                 + self.m03 * other.m32,
//             self.m00 * other.m03
//                 + self.m01 * other.m13
//                 + self.m02 * other.m23
//                 + self.m03 * other.m33,
//             self.m10 * other.m00
//                 + self.m11 * other.m10
//                 + self.m12 * other.m20
//                 + self.m13 * other.m30,
//             self.m10 * other.m01
//                 + self.m11 * other.m11
//                 + self.m12 * other.m21
//                 + self.m13 * other.m31,
//             self.m10 * other.m02
//                 + self.m11 * other.m12
//                 + self.m12 * other.m22
//                 + self.m13 * other.m32,
//             self.m10 * other.m03
//                 + self.m11 * other.m13
//                 + self.m12 * other.m23
//                 + self.m13 * other.m33,
//             self.m20 * other.m00
//                 + self.m21 * other.m10
//                 + self.m22 * other.m20
//                 + self.m23 * other.m30,
//             self.m20 * other.m01
//                 + self.m21 * other.m11
//                 + self.m22 * other.m21
//                 + self.m23 * other.m31,
//             self.m20 * other.m02
//                 + self.m21 * other.m12
//                 + self.m22 * other.m22
//                 + self.m23 * other.m32,
//             self.m20 * other.m03
//                 + self.m21 * other.m13
//                 + self.m22 * other.m23
//                 + self.m23 * other.m33,
//             self.m30 * other.m00
//                 + self.m31 * other.m10
//                 + self.m32 * other.m20
//                 + self.m33 * other.m30,
//             self.m30 * other.m01
//                 + self.m31 * other.m11
//                 + self.m32 * other.m21
//                 + self.m33 * other.m31,
//             self.m30 * other.m02
//                 + self.m31 * other.m12
//                 + self.m32 * other.m22
//                 + self.m33 * other.m32,
//             self.m30 * other.m03
//                 + self.m31 * other.m13
//                 + self.m32 * other.m23
//                 + self.m33 * other.m33,
//         )
//     }

//     // multiply vector
//     pub fn mul_vector(&self, other: &Vector4<T>) -> Vector4<T> {
//         Vector4::new(
//             self.m00 * other.x + self.m01 * other.y + self.m02 * other.z + self.m03 * other.w,
//             self.m10 * other.x + self.m11 * other.y + self.m12 * other.z + self.m13 * other.w,
//             self.m20 * other.x + self.m21 * other.y + self.m22 * other.z + self.m23 * other.w,
//             self.m30 * other.x + self.m31 * other.y + self.m32 * other.z + self.m33 * other.w,
//         )
//     }

//     // trace


//     // value_ptr
//     pub fn value_ptr(&self) -> *const T {
//         &self[0]
//     }

// }
// // // Into [[f32; 4]; 4]
// // impl Into<[[f32; 4]; 4]> for &Matrix4<f32> {
// //     fn into(self) -> [[f32; 4]; 4] {
// //         [
// //             [self.m00 as f32, self.m10 as f32, self.m20 as f32, self.m30 as f32],
// //             [self.m01 as f32, self.m11 as f32, self.m21 as f32, self.m31 as f32],
// //             [self.m02 as f32, self.m12 as f32, self.m22 as f32, self.m32 as f32],
// //             [self.m03 as f32, self.m13 as f32, self.m23 as f32, self.m33 as f32],
// //         ]
// //     }
// // }

// // impl From<[[f32; 4]; 4]> for Matrix4<f32> {
// //     fn from(mat4: [[f32; 4]; 4]) -> Self {
// //         Self { 
// //             m00: mat4[0][0], m10: mat4[0][1], m20: mat4[0][2], m30: mat4[0][3], 
// //             m01: mat4[1][0], m11: mat4[1][1], m21: mat4[1][2], m31: mat4[1][3], 
// //             m02: mat4[2][0], m12: mat4[2][1], m22: mat4[2][2], m32: mat4[2][3], 
// //             m03: mat4[3][0], m13: mat4[3][1], m23: mat4[3][2], m33: mat4[3][3] }
// //     }
// // }

// // // Into [[f32; 4]; 4]
// // impl Into<[f32; 16]> for Matrix4<f32> {
// //     fn into(self) -> [f32;16] {
// //         [
// //             self.m00 as f32, self.m10 as f32, self.m20 as f32, self.m30 as f32,
// //             self.m01 as f32, self.m11 as f32, self.m21 as f32, self.m31 as f32,
// //             self.m02 as f32, self.m12 as f32, self.m22 as f32, self.m32 as f32,
// //             self.m03 as f32, self.m13 as f32, self.m23 as f32, self.m33 as f32,
// //         ]
// //     }
// // }

// // // Into [[f32; 4]; 4]
// // impl Into<[f32; 16]> for &Matrix4<f32> {
// //     fn into(self) -> [f32;16] {
// //         [
// //             self.m00 as f32, self.m10 as f32, self.m20 as f32, self.m30 as f32,
// //             self.m01 as f32, self.m11 as f32, self.m21 as f32, self.m31 as f32,
// //             self.m02 as f32, self.m12 as f32, self.m22 as f32, self.m32 as f32,
// //             self.m03 as f32, self.m13 as f32, self.m23 as f32, self.m33 as f32,
// //         ]
// //     }
// // }
// // unsafe impl bytemuck::Pod for Matrix4<f32> {}
// // unsafe impl bytemuck::Zeroable for Matrix4<f32> {}


// pub type Matrix4f = Matrix4<f32>;
// pub type Matrix4i = Matrix4<i32>;
// pub type Matrix4u = Matrix4<u32>;



// // //  decompose transform matrix
// // pub fn decompose_transform_matrix(mat: &Matrix4f) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
// //     let translation = Vector3::new(mat.m03, mat.m13, mat.m23);
// //     let rotation = Quaternion::from_matrix(&mat);
// //     let rotation = rotation.get_euler();
// //     let scale = Vector3::new(
// //         (mat.m00 * mat.m00 + mat.m01 * mat.m01 + mat.m02 * mat.m02).sqrt(),
// //         (mat.m10 * mat.m10 + mat.m11 * mat.m11 + mat.m12 * mat.m12).sqrt(),
// //         (mat.m20 * mat.m20 + mat.m21 * mat.m21 + mat.m22 * mat.m22).sqrt(),
// //     );
// //     (translation, rotation, scale)
// // }

