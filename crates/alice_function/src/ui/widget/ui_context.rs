
use alice_core::math::Vector2f;

use crate::{paint::display::Layout, ui::{context::{ Context}, painter::Painter, id::Id, response::Response}};

use super::{ button::Button  , Widget, checkbox::Checkbox, label::Label, radio::Radio};



pub struct UiContext {
       layout:Layout,
   pub ctx:Context,
   pub auto_next_id:u64,
   pub id:Id
}


impl UiContext {
    pub fn new(id:Id,pos:Vector2f ,size:&Vector2f , ctx:Context ) -> Self {
        Self { id, layout: Layout::new(pos, size.x,size.y), ctx , auto_next_id:(id.with("auto").value()) }
    }

    pub fn allocate(&mut self , (w,h):(f32,f32)) -> ( Id , (f32,f32) ) {
        let id = Id::new(self.auto_next_id);
        self.auto_next_id = self.auto_next_id.wrapping_add(1);
        (id,self.layout.allocate((w,h)))
    }


    pub fn active_virtual_space(&mut self , (x,y):(f32,f32)) {
        self.layout.active_virtual_space((x,y))
    }

    pub fn allocate_virtual_space(&mut self , (w,h):(f32,f32))-> ( Id , (f32,f32) ) {
        let id = Id::new(self.auto_next_id);
        self.auto_next_id = self.auto_next_id.wrapping_add(1);
        (id,self.layout.allocate_virtual_space((w,h)))
    }



    pub fn label(&mut self, text:&str) -> Response{
        Label::new(text).ui(self)
    }

    pub fn button(&mut self , text:&str) -> Response{
        Button::new(text).ui(self)
    }


    pub fn checkbox(&mut self , checked:&mut bool, text:&str) -> Response{
        Checkbox::new(checked,text).ui(self)
    }

    pub fn same_line(&mut self){
        self.layout.force_same_line()
    }

    pub fn radio(&mut self , checked:bool , text:&str) -> Response {
        Radio::new(checked, text).ui(self)
    }

}