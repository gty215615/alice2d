
use std::{cell::RefCell, rc::Rc};


use alice_core::{math::Vector2f, color::Color};
use wgpu::util::DeviceExt;
use winit::{event::{Event, WindowEvent, KeyboardInput} , event_loop::{ControlFlow, EventLoop}};



use crate::{render::{rhi::Rhi, render_pipeline::{RenderPipeline, RenderPipelineInitInfo}},  paint::mesh::Mesh, ui::{layer::ClippedPrimitive, context::{ContextInstance, Context}, container::{panel::Panel, Container}}, window::{converter::{convert_element_state, convert_virtual_key_code, convert_mouse_button}, gui::Gui}, input::{input_state::InputContext, raw_input::RawInput, AliceInputState, input_system::InputSystem}};

use super::{window_config::MainWindowConfig, winit_window::WinitWindow};




pub fn window_run(){
    let event_loop = EventLoop::new();
    let window_config = MainWindowConfig::new("Alice (webgpu)".to_string(),1280,760);
    let window = WinitWindow::new(&event_loop,&window_config);

    
    let mut input_system = Rc::new(RefCell::new(InputSystem::new()));

    let mut gui = Gui::new(&window, input_system.clone());

    let rhi = Rc::new(RefCell::new(pollster::block_on(Rhi::new(&window))));

    let mut render_pipeline = RenderPipeline::new(RenderPipelineInitInfo { rhi:rhi.clone() });

    
    // let vb = rhi.borrow().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("Vertex Buffer"),
    //     contents: bytemuck::cast_slice(&mesh.vertices),
    //     usage: wgpu::BufferUsages::VERTEX,
    // });

    // let ib = rhi.borrow().device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("INDEX Buffer"),
    //     contents: bytemuck::cast_slice(&mesh.indices),
    //     usage: wgpu::BufferUsages::INDEX,
    // });

    // let cliped_primitive = ClippedPrimitive {
    //     vertices:vb,
    //     indics:ib,
    //     count:mesh.indices.len()
    // };

    // let cliped_primitives = vec![
    //     cliped_primitive
    // ];

        let mut index = 0;
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

                    let state = convert_element_state(*state);
                    let keycode = convert_virtual_key_code(virtual_keycode.unwrap());

 
                    
                }
            
          
                WindowEvent::MouseInput { state, button, .. } => {
                    input_system.borrow_mut().on_mouse_button_input(*state, *button);
                   

                }
                WindowEvent::MouseWheel { delta, .. } => {
                    // input.borrow_mut().on_mouse_wheel(*delta);
                    
                }
                WindowEvent::CursorMoved { position, .. } => {
                    input_system.borrow_mut().on_cursor_moved(*position);
                    
                }
                WindowEvent::CursorLeft { .. } => {
                    // engine.canvas.pointer_pos_in_points = None;
                    // engine.canvas.input.events.push(egui::Event::PointerGone);
               
                }
                // WindowEvent::TouchpadPressure {device_id, pressure, stage, ..  } => {} // TODO
                WindowEvent::Touch(touch) => {
                    // engine.canvas.on_touch(touch);
                    // match touch.phase {
                    //     winit::event::TouchPhase::Started
                    //     | winit::event::TouchPhase::Ended
                    //     | winit::event::TouchPhase::Cancelled => engine.canvas.ctx.wants_pointer_input(),
                    //     winit::event::TouchPhase::Moved => engine.canvas.ctx.is_using_pointer(),
                    // };
                }
                WindowEvent::ReceivedCharacter(ch) => {
                    // On Mac we get here when the user presses Cmd-C (copy), ctrl-W, etc.
                    // We need to ignore these characters that are side-effects of commands.
                    // let is_mac_cmd = cfg!(target_os = "macos")
                    //     && (engine.canvas.input.modifiers.ctrl || engine.canvas.input.modifiers.mac_cmd);
    
                    // if is_printable_char(*ch) && !is_mac_cmd {
                    //     engine.canvas.input
                    //         .events
                    //         .push(egui::Event::Text(ch.to_string()));
                    //     engine.canvas.ctx.wants_keyboard_input()
                    // } else {
                    //     false
                    // }
                }
          
                WindowEvent::Focused(has_focus) => {
                    // engine.canvas.input.has_focus = *has_focus;
                    // // We will not be given a KeyboardInput event when the modifiers are released while
                    // // the window does not have focus. Unset all modifier state to be safe.
                    // engine.canvas.input.modifiers = egui::Modifiers::default();
                  
                }
                WindowEvent::HoveredFile(path) => {
                    // engine.canvas.input.hovered_files.push(egui::HoveredFile {
                    //     path: Some(path.clone()),
                    //     ..Default::default()
                    // });
                   
                }
                WindowEvent::HoveredFileCancelled => {
                    // engine.canvas.input.hovered_files.clear();
                    
                }
                WindowEvent::DroppedFile(path) => {
                    // engine.canvas.input.hovered_files.clear();
                    // engine.canvas.input.dropped_files.push(egui::DroppedFile {
                    //     path: Some(path.clone()),
                    //     ..Default::default()
                    // });
                   
                }
                WindowEvent::ModifiersChanged(state) => {
                    // engine.canvas.input.modifiers.alt = state.alt();
                    // engine.canvas.input.modifiers.ctrl = state.ctrl();
                    // engine.canvas.input.modifiers.shift = state.shift();
                    // engine.canvas.input.modifiers.mac_cmd = cfg!(target_os = "macos") && state.logo();
                    // engine.canvas.input.modifiers.command = if cfg!(target_os = "macos") {
                    //     state.logo()
                    // } else {
                    //     state.ctrl()
                    // };
                
                }
                _ => {
                    // dbg!(event);
             
                }
                _ => {}
            },
            Event::RedrawRequested(_window_id) =>{                
                // engine.tick_per_frame()

                // input_ctx = std::mem::take(&mut input_ctx).begin_time(input.take());

                gui.begin_frame();
                
                input_system.borrow_mut().begin_frame();

      
                gui.tick_per_frame();

                let output = gui.end_frame();
      
                render_pipeline.draw(&output);
            }
            Event::MainEventsCleared => {
                // 除非手动请求，否则 RedrawRequested 只会触发一次
                // window.get_window().request_redraw()
             
                // let mut window_system = engine.ctx.as_ref().unwrap().window_system.borrow_mut();
               
          
                       
            }
            _ => {}
        }

    
    });


}




