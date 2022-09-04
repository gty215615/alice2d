use std::{rc::Rc, cell::RefCell};

use alice_core::math::Vector2f;

use crate::{ui::{context::Context, container::{panel::Panel, Container}, style::StyleSheet}, input::{ input_system::InputSystem}, paint::mesh::Mesh};

use super::winit_window::WinitWindow;


#[derive(Debug,Clone, Copy , PartialEq)]
pub enum RadioEnum {
    First,
    Second,
    Third
}

pub struct Gui {
    pub ctx:Context,
    pub cursor_pos:Vector2f ,
    pub checked:bool,
    pub radio:RadioEnum

}


impl Gui {
    pub fn new(window:&WinitWindow , input:Rc<RefCell<InputSystem>>) -> Self {
        Self { ctx: Context::new(window, input), cursor_pos: Vector2f::ZERO , checked:false , radio:RadioEnum::First }
    }

    pub fn begin_frame(&mut self){
        self.ctx.borrow_mut().begin_frame();
    }

    pub fn tick_per_frame(&mut self) {

            Panel::new("Checkbox Panel")
                .set_style(StyleSheet::MarginLeft(20.0))
                .set_style(StyleSheet::MarginTop(20.0))
                .set_style(StyleSheet::PaddingLeft(20.0))
                .set_style(StyleSheet::Width(400.0))
                .set_style(StyleSheet::Height(200.0))
                .show(&mut self.ctx,|ui| {
                    
                ui.label("Test Checkbox");
                ui.same_line();
                ui.checkbox(&mut self.checked,"checkbox0");
             
            });

            Panel::new("Checkbox Panel1")
                .set_style(StyleSheet::MarginLeft(20.0))
                .set_style(StyleSheet::MarginTop(20.0))
                .set_style(StyleSheet::PaddingLeft(20.0))
                .set_style(StyleSheet::Width(400.0))
                .set_style(StyleSheet::Height(200.0))
                .show(&mut self.ctx,|ui| {
                    
                ui.label("Test Checkbox");
                ui.same_line();
                ui.checkbox(&mut self.checked,"checkbox0");
              
            });

           


        //     Panel::new("Radio Panel")
        //     .set_style(StyleSheet::MarginLeft(20.0))
        //     .set_style(StyleSheet::MarginTop(20.0))
        //     .set_style(StyleSheet::PaddingLeft(20.0))
        //     .set_style(StyleSheet::Width(400.0))
        //     .set_style(StyleSheet::Height(200.0))
        //     .show(&mut self.ctx,|ui| {
                
        //     ui.label("Test Radio Group");
        //     ui.same_line();
        //     if ui.radio(self.radio == RadioEnum::First,"First").clicked() {
        //         self.radio = RadioEnum::First;
        //     };
        //     ui.same_line();
        //     if ui.radio(self.radio == RadioEnum::Second,"Second").clicked() {
        //         self.radio = RadioEnum::Second;
        //     };
           
        //     ui.same_line();
            
        //     if ui.radio(self.radio == RadioEnum::Third,"third").clicked() {
        //         self.radio = RadioEnum::Third;
        //     };
           
        // });
      
           

    }


    pub fn end_frame(&mut self) -> Vec<Mesh> {
        let meshs = self.ctx.borrow_mut().end_frame();

        meshs
    }
}
