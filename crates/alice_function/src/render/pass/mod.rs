use std::{rc::Rc, cell::RefCell};

use super::{rhi::Rhi};


pub struct RenderPassInitInfo {
   pub camera_group_layout: wgpu::BindGroupLayout
}


pub trait RenderPass {
    fn new(rhi:Rc<RefCell<Rhi>>) -> Self;
}



pub mod ui_pass;



