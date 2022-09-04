use crate::geometry::rect::Rect;

use super::{Modifiers, event::Event};


#[derive(Clone, Debug, PartialEq)]
pub struct RawInput {
    pub screen_rect:Option<Rect>,
    pub time:Option<f64>,
    pub predicted_dt:f32,
    pub modifiers:Modifiers,
    pub events:Vec<Event>,
}

impl Default for RawInput {
    fn default() -> Self {
        Self { 
            screen_rect: None, 
            time: None, 
            predicted_dt: 1.0 / 60.0, 
            modifiers: Modifiers::default(), 
            events: Vec::new() 
        }
    }
}

impl RawInput {
    pub fn take(&mut self) -> RawInput {
        // 移动事件 其余属性复制
        RawInput { 
            screen_rect: self.screen_rect.take(), 
            time: self.time.take(), 
            predicted_dt: self.predicted_dt, 
            modifiers: self.modifiers, 
            events: std::mem::take(&mut self.events)
        }
    }

    pub fn append(&mut self , newer:Self) {
        let Self {
            screen_rect,
            time,
            predicted_dt,
            modifiers,
            mut events
        } = newer;

        self.screen_rect = screen_rect.or(self.screen_rect);
        self.time = time.or(self.time);
        self.predicted_dt = predicted_dt;
        self.modifiers = modifiers;
        self.events.append(&mut events);
    }
}



