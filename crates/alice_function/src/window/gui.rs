use std::{rc::Rc, cell::RefCell};

use alice_core::math::Vector2f;

use crate::{ui::{context::Context, container::{panel::Panel, Container}, style::StyleSheet}, input::{ input_system::InputSystem}, paint::mesh::Mesh};

use super::winit_window::WinitWindow;



pub struct Gui {
    pub ctx:Context,
    pub cursor_pos:Vector2f ,
    pub checked:bool

}


impl Gui {
    pub fn new(window:&WinitWindow , input:Rc<RefCell<InputSystem>>) -> Self {
        Self { ctx: Context::new(window, input), cursor_pos: Vector2f::ZERO , checked:false }
    }

    pub fn begin_frame(&mut self){
        self.ctx.borrow_mut().begin_frame();
    }

    pub fn tick_per_frame(&mut self) {

            Panel::default()
                .set_style(StyleSheet::MarginLeft(20.0))
                .set_style(StyleSheet::MarginTop(20.0))
                .set_style(StyleSheet::PaddingLeft(20.0))
                .set_style(StyleSheet::Width(400.0))
                .set_style(StyleSheet::Height(200.0))
                .show(&mut self.ctx,|ui| {
          
                ui.checkbox(&mut self.checked,"checkbox checkbox checkbox");
                ui.checkbox(&mut self.checked,"checkbox checkbox checkbox");
                ui.checkbox(&mut self.checked,"checkbox checkbox checkbox");
                ui.checkbox(&mut self.checked,"checkbox checkbox checkbox");
                ui.checkbox(&mut self.checked,"checkbox checkbox checkbox");
                ui.checkbox(&mut self.checked,"checkbox checkbox checkbox");


            });


      
           

    }


    pub fn end_frame(&mut self) -> Vec<Mesh> {
        let meshs = self.ctx.borrow_mut().end_frame();

        meshs
    }
}
