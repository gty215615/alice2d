use alice_core::math::{Vector2, Vector2u};
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

    pub fn get_size(&self) -> Vector2u {
        let size = self.window.inner_size();
        Vector2u::new(size.width, size.height)
    }

 
    pub fn run(&self) {}
}


