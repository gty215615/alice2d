use self::ui_context::UiContext;

pub mod button;

pub mod checkbox;

pub mod ui_context;

pub mod label;

pub mod icon;

pub trait Widget {
    fn ui(&mut self, ctx:&mut UiContext);
}


