
use std::{rc::Rc, cell::RefCell, iter};






use crate::{ui::layer::ClippedPrimitive, paint::mesh::Mesh};

use super::{ rhi::Rhi, pass::{ui_pass::UIPass}};


pub struct RenderPipelineInitInfo {
    pub rhi: Rc<RefCell<Rhi>>,

}


pub struct RenderPipeline {

    ui_pass:          UIPass,
    rhi:Rc<RefCell<Rhi>>,
 
}


impl RenderPipeline {
    pub fn new(init_info:RenderPipelineInitInfo) -> Self {
        // let mut material_layout = Vec::with_capacity(2);
        let rhi = init_info.rhi.clone();

        let mut ui_pass =  UIPass::new(rhi.clone(),1);

        Self {
           
            ui_pass,
            rhi,
 
        }
    }


   

    pub fn draw(&mut self , paint_jobs:&[Mesh]){
        // render_pass:&mut wgpu::RenderPass<'pass> ,  rhi:Rc<RefCell<Rhi>>, 
     
        let mut encoder = self.rhi.borrow()
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        let output = self.rhi.borrow().surface.get_current_texture().unwrap();

     
            // self.main_camera_pass.draw(&mut render_pass, self.rhi.clone(),render_scene.clone(),&self.webgpu_scene);
            
                
                // let (textures_delta,paint_job) = &render_scene.borrow().frame;
        
                self.ui_pass.draw(self.rhi.clone(),&mut encoder,paint_jobs, &output);
           
            // self.pipeline
            //     .iter()
            //     .for_each(|pipeline| pipeline.submit(&mut render_pass));


        
            self.rhi.borrow().queue.submit(iter::once(encoder.finish()));
                    output.present();
    
     

    }
    // pub fn draw(&self , render_scene:&RenderScene)
    


}
























