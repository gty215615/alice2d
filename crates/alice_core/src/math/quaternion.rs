
// use super::{matrix4::Matrix4, vector3::Vector3};
// use std::{f32::consts::PI, ops::{Add, Sub, Div, Neg, Mul}};
// #[allow(dead_code)]
// type Quat = Quaternion;





// #[derive(Default,Debug ,Clone, Copy)]
// pub struct Quaternion {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
//     pub w: f32,
// }

// impl Quaternion {
//     pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
//         Quaternion { x, y, z, w }
//     }

//     pub fn identity() -> Quaternion {
//         Quaternion {
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//             w: 1.0,
//         }
//     }

//     pub fn from_axis_angle<T>(axis: Vector3<T>, angle: T) -> Quaternion
//     where  T: Copy
//     + Default
//     + PartialOrd
//     + Add<Output = T>
//     + Sub<Output = T>
//     + Mul<Output = T>
//     + Div<Output = T>
//     + Neg<Output = T>
//     + PartialEq
//     + From<u32>
//     + From<f32>
//     {
//         let half_angle = angle / 2.0;
//         let sin_half_angle = f32::sin(half_angle);
//         Quaternion {
//             x: axis.x * sin_half_angle,
//             y: axis.y * sin_half_angle,
//             z: axis.z * sin_half_angle,
//             w: f32::cos(half_angle),
//         }
//     }

//     pub fn from_euler(x: f32, y: f32, z: f32) -> Quaternion {
//         let q = Quaternion::identity();
//         q.set_euler(x, y, z)
//     }

//     pub fn set_euler(mut self, x: f32, y: f32, z: f32) -> Quaternion {
//         let half_x = x / 2.0;
//         let half_y = y / 2.0;
//         let half_z = z / 2.0;
//         let sin_x = f32::sin(half_x);
//         let sin_y = f32::sin(half_y);
//         let sin_z = f32::sin(half_z);
//         let cos_x = f32::cos(half_x);
//         let cos_y = f32::cos(half_y);
//         let cos_z = f32::cos(half_z);
//         self.x = sin_x * cos_y * cos_z - cos_x * sin_y * sin_z;
//         self.y = cos_x * sin_y * cos_z + sin_x * cos_y * sin_z;
//         self.z = cos_x * cos_y * sin_z - sin_x * sin_y * cos_z;
//         self.w = cos_x * cos_y * cos_z + sin_x * sin_y * sin_z;
//         self
//     }

//     // get euler angles from quaternion
//     pub fn get_euler<T>(&self) -> Vector3<T> 
//     where  T: Copy
//     + Default
//     + PartialOrd
//     + Add<Output = T>
//     + Sub<Output = T>
//     + Mul<Output = T>
//     + Div<Output = T>
//     + Neg<Output = T>
//     + PartialEq
//     + From<u32>
//     + From<f32>
//     {
//         let sqw = self.w * self.w;
//         let sqx = self.x * self.x;
//         let sqy = self.y * self.y;
//         let sqz = self.z * self.z;
//         let unit = sqx + sqy + sqz + sqw; // if normalised is one, otherwise is correction factor
//         let test = self.x * self.y + self.z * self.w;
//         let mut heading = Vector3::new(0.0, 0.0, 0.0);
//         if test > 0.499 * unit {
//             // singularity at north pole
//             heading.x = 2.0 * f32::atan2(self.x, self.w);
//             heading.y = PI / 2.0;
//             heading.z = 0.0;
//             return heading;
//         }
//         if test < -0.499 * unit {
//             // singularity at south pole
//             heading.x = -2.0 * f32::atan2(self.x, self.w);
//             heading.y = -PI / 2.0;
//             heading.z = 0.0;
//             return heading;
//         }
//         heading.x = f32::atan2(2.0 * self.y * self.w - 2.0 * self.x * self.z, sqx - sqy - sqz + sqw);
//         heading.y = f32::asin(2.0 * test / unit);
//         heading.z = f32::atan2(2.0 * self.x * self.w - 2.0 * self.y * self.z, -sqx + sqy - sqz + sqw);
//         heading
//     }

//     pub fn set_axis_angle<T>(mut self, axis: Vector3<T>, angle: T) -> Quaternion 
//     where  T: Copy
//     + Default
//     + PartialOrd
//     + Add<Output = T>
//     + Sub<Output = T>
//     + Mul<Output = T>
//     + Div<Output = T>
//     + Neg<Output = T>
//     + PartialEq
//     + From<u32>
//     + From<f32>
//     {
//         let half_angle = angle / 2.0;
//         let sin_half_angle = f32::sin(half_angle);
//         self.x = axis.x * sin_half_angle;
//         self.y = axis.y * sin_half_angle;
//         self.z = axis.z * sin_half_angle;
//         self.w = f32::cos(half_angle);
//         self
//     }

//     pub fn set_from_rotation_euler(self, x: f32, y: f32, z: f32) -> Quaternion {
//         self.set_euler(x, y, z)
//     }

//     pub fn set_from_rotation_axis_angle<T>(self, axis: Vector3<T>, angle: T) -> Quaternion 
//     where  T: Copy
//     + Default
//     + PartialOrd
//     + Add<Output = T>
//     + Sub<Output = T>
//     + Mul<Output = T>
//     + Div<Output = T>
//     + Neg<Output = T>
//     + PartialEq
//     + From<u32>
//     + From<f32>
//     {
//         self.set_axis_angle(axis, angle)
//     }

//     // get rotation axis and angle from quaternion
//     pub fn get_rotation_axis_angle<T>(&self) -> (Vector3<T>, T) 
//     where  T: Copy
//     + Default
//     + PartialOrd
//     + Add<Output = T>
//     + Sub<Output = T>
//     + Mul<Output = T>
//     + Div<Output = T>
//     + Neg<Output = T>
//     + PartialEq
//     + From<u32>
//     + From<f32>
//     {
//         let half_angle = f32::acos(self.w) * 2.0;
//         let sin_half_angle = f32::sin(half_angle);
//         let axis = Vector3::new(self.x / sin_half_angle, self.y / sin_half_angle, self.z / sin_half_angle);
//         (axis, half_angle)
//     }



//     pub fn from_matrix<T>( m:&Matrix4<T>) -> Quaternion 
//     where  T: Copy
//     + Default
//     + PartialOrd
//     + Add<Output = T>
//     + Sub<Output = T>
//     + Mul<Output = T>
//     + Div<Output = T>
//     + Neg<Output = T>
//     + PartialEq
//     + From<u32>
//     + From<f32>
//     {
//         let trace = m.m00 + m.m11 + m.m22;
//         let mut q = Quaternion::identity();
//         if trace > 0.0 {
//             let s = f32::sqrt(trace + 1.0) * 2.0;
//             q.w = 0.25 * s;
//             q.x = (m.m21 - m.m12) / s;
//             q.y = (m.m02 - m.m20) / s;
//             q.z = (m.m10 - m.m01) / s;
//         } else if m.m00 > m.m11 && m.m00 > m.m22 {
//             let s = f32::sqrt(1.0 + m.m00 - m.m11 - m.m22) * 2.0;
//             q.w = (m.m21 - m.m12) / s;
//             q.x = 0.25 * s;
//             q.y = (m.m01 + m.m10) / s;
//             q.z = (m.m02 + m.m20) / s;
//         } else if m.m11 > m.m22 {
//             let s = f32::sqrt(1.0 + m.m11 - m.m00 - m.m22) * 2.0;
//             q.w = (m.m02 - m.m20) / s;
//             q.x = (m.m01 + m.m10) / s;
//             q.y = 0.25 * s;
//             q.z = (m.m12 + m.m21) / s;
//         } else {
//             let s = f32::sqrt(1.0 + m.m22 - m.m00 - m.m11) * 2.0;
//             q.w = (m.m10 - m.m01) / s;
//             q.x = (m.m02 + m.m20) / s;
//             q.y = (m.m12 + m.m21) / s;
//             q.z = 0.25 * s;
//         }

//         q
//     }

//     pub fn set_from_quaternion(mut self, q: Quaternion) -> Quaternion {
//         self.x = q.x;
//         self.y = q.y;
//         self.z = q.z;
//         self.w = q.w;
//         self
//     }

//     pub fn to_matrix<T>(&self) -> Matrix4<T> 
//     where  T: Copy
//     + Default
//     + PartialOrd
//     + Add<Output = T>
//     + Sub<Output = T>
//     + Mul<Output = T>
//     + Div<Output = T>
//     + Neg<Output = T>
//     + PartialEq
//     + From<u32>
//     + From<f32>
//     {
//         let xx = self.x * self.x;
//         let xy = self.x * self.y;
//         let xz = self.x * self.z;
//         let xw = self.x * self.w;
//         let yy = self.y * self.y;
//         let yz = self.y * self.z;
//         let yw = self.y * self.w;
//         let zz = self.z * self.z;
//         let zw = self.z * self.w;
//         Matrix4::new(
//             1.0 - 2.0 * (yy + zz),
//             2.0 * (xy + zw),
//             2.0 * (xz - yw),
//             0.0,
//             2.0 * (xy - zw),
//             1.0 - 2.0 * (xx + zz),
//             2.0 * (yz + xw),
//             0.0,
//             2.0 * (xz + yw),
//             2.0 * (yz - xw),
//             1.0 - 2.0 * (xx + yy),
//             0.0,
//             0.0,
//             0.0,
//             0.0,
//             1.0,
//         )
//     }

  
// }
