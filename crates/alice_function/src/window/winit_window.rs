use alice_core::math::Vector2;
use winit::{
    dpi::PhysicalSize,
    
    event_loop::{EventLoop},
    window::{ Window, WindowBuilder},
};



use super::window_config::MainWindowConfig;



pub struct WinitWindow {
    window: Window,
    
}

impl WinitWindow {
    pub fn new(event_loop: &EventLoop<()>,config:&MainWindowConfig) -> Self {

        let window            = WindowBuilder::new()
                                            .with_inner_size(PhysicalSize::new(config.width,config.height))
                                            .with_title(&config.title)
                                            .build(&event_loop)
                                            .unwrap();

        Self { window  }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn get_size(&self) -> Vector2<f32> {
        let size = self.window.inner_size();
        Vector2::new(size.width as f32, size.height as f32)
    }

 
    pub fn run(&self) {}
}


