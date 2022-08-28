use std::{rc::Rc, cell::RefCell};


use alice_core::math::{Vector2f, Vector2u};
use winit::window::Window;

use crate::window::winit_window::WinitWindow;




pub struct RhiInitInfo {
    window:Window
}


pub struct Rhi {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: Vector2u,

 
}


impl Rhi {
    pub async fn new(window:&WinitWindow)-> Self {

        let window = window.get_window();
        let size = window.inner_size();


        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = Self::create_adapter(&instance, &surface).await;
        let (device, queue) = Self::create_device_and_queue(&adapter).await;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size: Vector2u::new(size.width , size.height)
        }

    }


    

    async fn create_adapter(instance:&wgpu::Instance , surface:&wgpu::Surface) -> wgpu::Adapter {
        instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap()
    }

    async fn create_device_and_queue(adapter:&wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
            },
            None, // Trace path
        )
        .await
        .unwrap();
        (device, queue)
    }
}







