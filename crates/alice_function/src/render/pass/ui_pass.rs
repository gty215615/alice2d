#![allow(unsafe_code)]

use std::cell::RefCell;
use std::iter;
use std::rc::Rc;


use wgpu::util::DeviceExt;
use wgpu::{self, CommandEncoder};


use crate::paint::layer::{PaintJob, ClippedPrimitive};
use crate::paint::mesh::{Vertex, Mesh};
use crate::render::loader::ShaderLoader;
use crate::render::rhi::Rhi;




/// Enum for selecting the right buffer type.
#[derive(Debug)]
enum BufferType {
    Uniform,
    Index,
    Vertex,
}

/// Information about the screen used for rendering.
pub struct ScreenDescriptor {
    /// Size of the window in physical pixels.
    pub size_in_pixels: [u32; 2],

    /// HiDPI scale factor (pixels per point).
    pub pixels_per_point: f32,
}

impl ScreenDescriptor {
    /// size in "logical" points
    fn screen_size_in_points(&self) -> [f32; 2] {
        [
            self.size_in_pixels[0] as f32 / self.pixels_per_point,
            self.size_in_pixels[1] as f32 / self.pixels_per_point,
        ]
    }
}

/// Uniform buffer used when rendering.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct UniformBuffer {
    screen_size_in_points: [f32; 2],
    // Uniform buffers need to be at least 16 bytes in WebGL.
    // See https://github.com/gfx-rs/wgpu/issues/2072
    _padding: [u32; 2],
}


unsafe impl bytemuck::Pod for UniformBuffer {}
unsafe impl bytemuck::Zeroable for UniformBuffer {}


/// Render pass to render a egui based GUI.
pub struct UIPass {
    render_pipeline: wgpu::RenderPipeline,


}

impl UIPass {
    /// Creates a new render pass to render a egui UI.
    ///
    /// If the format passed is not a *Srgb format, the shader will automatically convert to `sRGB` colors in the shader.
    pub fn new(
        rhi:Rc<RefCell<Rhi>>,
        msaa_samples: u32,
    ) -> Self {
        let shader = "assets/shader/ui_shader.wgsl";

        let shader = ShaderLoader::load(&rhi.borrow().device,shader);
        let pipeline_layout = rhi.borrow().device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("egui_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = rhi.borrow().device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("egui_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                entry_point: "vs_main",
                module: &shader,
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 8 * 4,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    // 0: vec2 position
                    // 1: vec2 texture coordinates
                    // 2: uint color
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4],
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                unclipped_depth: false,
                conservative: false,
                cull_mode: None,
                front_face: wgpu::FrontFace::default(),
                polygon_mode: wgpu::PolygonMode::default(),
                strip_index_format: None,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                alpha_to_coverage_enabled: false,
                count: msaa_samples,
                mask: !0,
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: rhi.borrow().config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::OneMinusDstAlpha,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
     
        Self {
            render_pipeline,
   
 
       
      
        }
    }

    pub fn draw(&mut self ,rhi:Rc<RefCell<Rhi>>, encoder: &mut CommandEncoder, paint_jobs:&[ClippedPrimitive] , output:&wgpu::SurfaceTexture){
                // Upload all resources for the GPU.]
           
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                   
                    {

                        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 0.1,
                                        g: 0.2,
                                        b: 0.3,
                                        a: 1.0,
                                    }),
                                    store: true,
                                },
                            })],
                            depth_stencil_attachment:None
                            // depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            //     view: &self.depth_texture_view,
                            //     depth_ops: Some(wgpu::Operations {
                            //         load: wgpu::LoadOp::Clear(1.0),
                            //         store: true,
                            //     }),
                            //     stencil_ops: None,
                            // }),
                        });

                        render_pass.set_pipeline(&self.render_pipeline);
                 
                        for job in paint_jobs {
                            
                            render_pass.set_vertex_buffer(0, job.vertices.slice(..));
                            render_pass.set_index_buffer(job.indics.slice(..), wgpu::IndexFormat::Uint16);

                            render_pass.draw_indexed(0..job.count as u32, 0, 0..1);
                        }
                    }
        

        
           
    }
    

}