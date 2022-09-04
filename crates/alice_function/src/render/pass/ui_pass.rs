#![allow(unsafe_code)]

use std::cell::RefCell;
use std::iter;
use std::num::NonZeroU32;
use std::path::Path;
use std::rc::Rc;


use image::{GenericImageView, DynamicImage, ImageBuffer, Rgb, Rgba};
use wgpu::util::DeviceExt;
use wgpu::{self, CommandEncoder};


use crate::paint::mesh::{Vertex, Mesh};
use crate::render::loader::ShaderLoader;
use crate::render::rhi::Rhi;
use crate::ui::layer::ClippedPrimitive;

use super::ui_data::BufferData;




/// Enum for selecting the right buffer type.
#[derive(Debug)]
enum BufferType {
    // Uniform,
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

    group:wgpu::BindGroup,
    index_buffers: Vec<BufferData>,
    vertex_buffers: Vec<BufferData>,
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

        let (layout , group) = create_texture_group(rhi.clone(),"test.png" , "font atlas");

        let pipeline_layout = rhi.borrow().device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("egui_pipeline_layout"),
            bind_group_layouts: &[&layout],
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
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
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
            group,
            index_buffers: Vec::with_capacity(64),
            vertex_buffers: Vec::with_capacity(64),
       
      
        }
    }

    pub fn draw(&mut self ,rhi:Rc<RefCell<Rhi>>, encoder: &mut CommandEncoder, paint_jobs:&[Mesh] , output:&wgpu::SurfaceTexture){
                // Upload all resources for the GPU.]

                self.update_buffers(rhi.clone(), paint_jobs);

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
                                        r: 0.0,
                                        g: 0.0,
                                        b: 0.0,
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
                        render_pass.set_bind_group(0, &self.group, &[]);


                        let mut index_buffers = self.index_buffers.iter();
                        let mut vertex_buffers = self.vertex_buffers.iter();
                     

                   
                        for job in paint_jobs {

                            let index_buffer = index_buffers.next().unwrap();
                            let vertex_buffer = vertex_buffers.next().unwrap();
        
                    
                            render_pass.set_vertex_buffer(0, vertex_buffer.buffer.slice(..));
                            render_pass.set_index_buffer(index_buffer.buffer.slice(..), wgpu::IndexFormat::Uint16);
                            render_pass.draw_indexed(0..job.indices.len() as u32, 0, 0..1);
                        }

                    }
        

        
           
    }
    

      /// Uploads the uniform, vertex and index data used by the render pass.
    /// Should be called before `execute()`.
    pub fn update_buffers(
        &mut self,
        rhi:Rc<RefCell<Rhi>>,
        paint_jobs: &[Mesh]
    ) {

    
        let mut mesh_idx = 0;
        for mesh in paint_jobs.iter() {
                let data: &[u8] = bytemuck::cast_slice(&mesh.indices);
                    if mesh_idx < self.index_buffers.len() {
                        self.update_buffer(rhi.clone(), &BufferType::Index, mesh_idx, data);
                    } else {
                        let buffer = rhi.borrow().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("egui_index_buffer"),
                            contents: data,
                            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                        });
                        self.index_buffers.push(BufferData {
                            buffer,
                            size: data.len(),
                        });
                    }

                    let data: &[u8] = bytemuck::cast_slice(&mesh.vertices);
                    if mesh_idx < self.vertex_buffers.len() {
                        self.update_buffer(rhi.clone(),  &BufferType::Vertex, mesh_idx, data);
                    } else {
                        let buffer = rhi.borrow().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("egui_vertex_buffer"),
                            contents: data,
                            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                        });

                        self.vertex_buffers.push(BufferData {
                            buffer,
                            size: data.len(),
                        });
                    }

                    mesh_idx += 1;
        }
    }


    /// Updates the buffers used by egui. Will properly re-size the buffers if needed.
    fn update_buffer(
        &mut self,
        rhi:Rc<RefCell<Rhi>>,
        buffer_type: &BufferType,
        index: usize,
        data: &[u8],
    ) {
        let (buffer, storage, label) = match buffer_type {
            BufferType::Index => (
                &mut self.index_buffers[index],
                wgpu::BufferUsages::INDEX,
                "egui_index_buffer",
            ),
            BufferType::Vertex => (
                &mut self.vertex_buffers[index],
                wgpu::BufferUsages::VERTEX,
                "egui_vertex_buffer",
            ),
 
        };

        if data.len() > buffer.size {
            buffer.size = data.len();
            buffer.buffer = rhi.borrow().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice(data),
                usage: storage | wgpu::BufferUsages::COPY_DST,
            });
        } else {
           rhi.borrow().queue.write_buffer(&buffer.buffer, 0, data);
        }
    }
}

pub fn create_texture_group(rhi:Rc<RefCell<Rhi>>,path:&str,label: &str) -> (wgpu::BindGroupLayout,wgpu::BindGroup) {
    let (texture,view,sampler) = from_path(rhi.clone(), path, label);
    let pbr_material_layout = rhi.borrow().device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
        wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
        ],
        label: Some("bind base color texture layout"),
    });
    let  pbr_material_group = rhi.borrow().device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &pbr_material_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&view),
            }
      
        ],
        label: Some("diffuse_bind_group"),
    });

    (pbr_material_layout,pbr_material_group)
}

pub fn from_path(
    rhi:Rc<RefCell<Rhi>> , 
    path:&str,
    label: &str,) -> (wgpu::Texture,wgpu::TextureView,wgpu::Sampler) {
        let mut img;
        if path.is_empty() {
            let mut buffer:ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(1, 1);
        
            buffer.put_pixel(0, 0, Rgba([255, 255, 255, 255]));
            img = DynamicImage::ImageRgba8(buffer);
        }else{
            img = image::open(&Path::new(path)).expect("load texture failed");
            // img = img.flipv();
          
        }
       
        from_image(rhi, &img, Some(label))
}

pub fn from_image(
    rhi:Rc<RefCell<Rhi>> , 
    img: &image::DynamicImage,
    label: Option<&str>,
) -> (wgpu::Texture,wgpu::TextureView,wgpu::Sampler) {
    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };
    let texture = rhi.borrow().device.create_texture(&wgpu::TextureDescriptor {
        label,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
    });

    rhi.borrow().queue.write_texture(
        wgpu::ImageCopyTexture {
            aspect: wgpu::TextureAspect::All,
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: NonZeroU32::new(4 * dimensions.0),
            rows_per_image: NonZeroU32::new(dimensions.1),
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = rhi.borrow().device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    (texture,
        view,
        sampler)
}
