use self::ui_context::UiContext;

use super::response::Response;

pub mod button;

pub mod checkbox;

pub mod radio;

pub mod ui_context;

pub mod label;

pub mod text_edit;

pub mod icon;

pub trait Widget {
    fn ui(&mut self, ctx:&mut UiContext) -> Response;
}


