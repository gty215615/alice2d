
use super::{context::{ Context}, widget::ui_context::UiContext};


pub mod panel;
mod Panel;



pub trait Container {
    fn show(&mut self, ctx:&mut Context , render_children:impl FnOnce(&mut UiContext) );
}