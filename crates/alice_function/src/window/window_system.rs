
use std::{cell::RefCell, rc::Rc};


use alice_core::{math::Vector2f, color::Color};
use wgpu::util::DeviceExt;
use winit::{event::{Event, WindowEvent, KeyboardInput} , event_loop::{ControlFlow, EventLoop}};



use crate::{render::{rhi::Rhi, render_pipeline::{RenderPipeline, RenderPipelineInitInfo}}, paint::{context::{Context, stroke_line, stroke_path}, path::{Path, Stroke, PathType}, mesh::Mesh, layer::ClippedPrimitive}};

use super::{window_config::MainWindowConfig, winit_window::WinitWindow};




pub fn window_run(){
    let event_loop = EventLoop::new();
    let window_config = MainWindowConfig::new("Alice (webgpu)".to_string(),1280,760);
    let window = WinitWindow::new(&event_loop,&window_config);


    let rhi = Rc::new(RefCell::new(pollster::block_on(Rhi::new(&window))));

    let mut render_pipeline = RenderPipeline::new(RenderPipelineInitInfo { rhi:rhi.clone() });


    let mut path = Path::new();
    // path.add_line_segment([Vector2f::new(100.0,100.0) , Vector2f::new(500.0,100.0)]);


    path.add_line_closed(vec![
        Vector2f::new(100.0,100.0) ,
        Vector2f::new(100.0,300.0) ,
        Vector2f::new(300.0,300.0) ,
        Vector2f::new(300.0,100.0) ,
    
    ]);

    let mut mesh = Mesh::default();


    stroke_path(path.0.as_slice() , Stroke {
        width:15.0,
        color:Color::BLUE
    } , PathType::Closed, &mut mesh);

    for v in mesh.vertices.iter() {
        println!("{:?}",v.pos);
    }
    println!("indics = {:?}",mesh.indices);
   
    let vb = rhi.borrow().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&mesh.vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let ib = rhi.borrow().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("INDEX Buffer"),
        contents: bytemuck::cast_slice(&mesh.indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    let cliped_primitive = ClippedPrimitive {
        vertices:vb,
        indics:ib,
        count:mesh.indices.len()
    };

    let cliped_primitives = vec![
        cliped_primitive
    ];


    event_loop.run(move |event, _, control_flow| {
        
        match event {
            Event::WindowEvent {
                ref event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode,
                            ..
                        },
                    ..
                } => {

                    // let state = convert_element_state(*state);
                    // let keycode = convert_virtual_key_code(virtual_keycode.unwrap());

                  
                    
                }
            
          
              
                _ => {}
            },
            Event::RedrawRequested(_window_id) =>{                
                // engine.tick_per_frame()
                render_pipeline.draw(&cliped_primitives);
            }
            Event::MainEventsCleared => {
                // 除非手动请求，否则 RedrawRequested 只会触发一次

                // if index < 2 {
              
                //     index += 1;
                // }
             
                // let mut window_system = engine.ctx.as_ref().unwrap().window_system.borrow_mut();
                // window_system.request_draw();   
          
                       
            }
            _ => {}
        }

    
    });


}




