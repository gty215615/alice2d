use super::shape::Shape;





pub struct PaintList(Vec<Shape>);


pub struct ClippedPrimitive {
   pub indics:wgpu::Buffer,
   pub vertices:wgpu::Buffer,
   pub count:usize
}
pub struct PaintJob(Vec<ClippedPrimitive>);

pub struct Layer {
    pub z_index:i32,

}







