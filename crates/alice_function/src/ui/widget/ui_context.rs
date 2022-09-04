
use alice_core::math::Vector2f;

use crate::{paint::display::Layout, ui::{context::{ Context}, painter::Painter}};

use super::{ button::Button  , Widget, checkbox::Checkbox};



pub struct UiContext {
   pub layout:Layout,
   pub ctx:Context
}


impl UiContext {
    pub fn new(pos:Vector2f ,size:&Vector2f , ctx:Context ) -> Self {
        Self { layout: Layout::new(pos, size.x,size.y), ctx }
    }



    pub fn button(&mut self , text:&str){
        Button::new(text).ui(self)
    }


    pub fn checkbox(&mut self , checked:&mut bool, text:&str){
        Checkbox::new(checked,text).ui(self)
    }

}