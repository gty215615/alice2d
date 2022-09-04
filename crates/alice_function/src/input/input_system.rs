use std::{cell::RefCell, rc::Rc};

use alice_core::math::Vector2f;

use crate::window::converter::{convert_mouse_button, convert_element_state};

use super::{raw_input::RawInput, input_state::InputContext, event::Event, AliceInputState};



pub struct InputSystem {
    pub ctx:InputContext, 
    pub raw_input: RawInput , 
    pub cursor_pos:Option<Vector2f> ,
}

impl InputSystem {
    pub fn new() -> Self {
        Self { ctx: InputContext::default(), raw_input: Default::default(), cursor_pos: None }
    }

    pub fn on_mouse_button_input(
        &mut self,
        state: winit::event::ElementState,
        button: winit::event::MouseButton,
    ) {
        if let Some(pos) = self.cursor_pos {
                let button = convert_mouse_button(button);
                let state = convert_element_state(state);
                self.raw_input.events.push(Event::PointerButton {
                    pos,
                    button,
                    pressed:state == AliceInputState::Pressed,
                    modifiers: self.raw_input.modifiers,
                });
        }
    }

    pub fn begin_frame(&mut self) {
        // input_ctx = std::mem::take(&mut input_ctx).begin_time(input.take());
        self.ctx = std::mem::take(&mut self.ctx).begin_time(self.raw_input.take());
    }

    pub fn on_cursor_moved(&mut self, pos_in_pixels: winit::dpi::PhysicalPosition<f64>) {
        self.cursor_pos = Some(Vector2f::new(pos_in_pixels.x as f32,pos_in_pixels.y as f32));
        self.raw_input
        .events
        .push(Event::PointerMoved(self.cursor_pos.unwrap()));
    }
}


