use alice_core::allocator::GenerateId;

use crate::paint::{shape::Shape, mesh::Mesh};


pub const MAX_LAYER_NUM:usize = 3;

pub enum LayerId {
    Background = 0,
    Document = 1,
    Popup = 2
}

pub struct PaintList(Vec<Shape>);


pub struct ClippedPrimitive {
   pub indics:wgpu::Buffer,
   pub vertices:wgpu::Buffer,
   pub count:usize,
}
pub struct PaintJob(pub(crate) Vec<ClippedPrimitive>);

pub struct Layer {
    pub z_index:i32,
    commands:PaintList,
}

impl Layer {
    pub fn new()->Self {
        Self { z_index: 0, commands: PaintList(Vec::new())  }
    }

    pub fn add_command(&mut self, shape:impl Into<Shape> ) {
        self.commands.0.push(shape.into());
    }

    pub fn drain(&mut self) -> Vec<Shape> {
        self.commands.0.drain(..).collect::<Vec<Shape>>()
    }
}







