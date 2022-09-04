use crate::{input::{input_state::InputContext, raw_input::RawInput, KeyCode, Modifiers, event::Event}, geometry::rect::Rect};

use super::{utils::id_type_map::IdTypeMap, id::{Id, IdMap}};



#[derive(Debug,Clone,Default)]
pub struct Store {
   pub data:IdTypeMap,
   pub interaction:Interaction,
   popup:Option<Id>
}


impl Store {
    pub fn begin_frame(&mut self , prev_input:&InputContext , new_input:&RawInput) {
        self.interaction.begin_frame(prev_input, new_input);
        if !prev_input.pointer.any_down() {

        }
    }

    pub fn is_popup(&self,id:Id) -> bool {
        self.popup == Some(id)
    }

    pub fn end_frame(&mut self , input:&InputContext , used_id:IdMap<Rect>){
        self.interaction.focus.end_frame(used_id);
    }

    pub fn want_focus(&mut self , id:Id) {
        self.interaction.focus.id_next_frame = Some(id);
    }
}


#[derive(Debug,Clone,Default)]
pub struct Interaction {
    pub click_id:Option<Id>,
    pub hover_id:Option<Id>,
    pub focus:Focus,
    pub drag_id:Option<Id>,
    pub click_interest:bool,
    pub drag_interest:bool,
}


impl Interaction {

    pub fn is_using_pointer(&self)-> bool {
        self.click_id.is_some() || self.drag_id.is_some()
    }

    pub fn begin_frame(&mut self , prev_input:&InputContext , new_input:&RawInput) {
        self.click_interest = false;
        self.drag_interest = false;

        if !prev_input.pointer.may_be_any_button_be_click() {
            self.click_id = None;
        }

        if !prev_input.pointer.any_down() || prev_input.pointer.latest_pos().is_none(){
            self.click_id = None;
            self.drag_id = None;
        }

        self.focus.begin_frame(new_input);

    }
}



#[derive(Debug,Clone,Default)]
pub struct Focus {
     id:Option<Id>,
    // 上一帧聚焦的物件
    id_prev_frame:Option<Id>,
    // 设置的物件下一帧聚焦
    id_next_frame:Option<Id>,
    // 如果可能,下一个想要获取焦点的物件自动聚焦 , eg: 按下 Tab键
    want_focus_at_next_frame:bool,
    // 上一个聚集的部件
    last_interested:Option<Id>,
    // 聚焦锁定, 按下 tab 键不会失去焦点
    is_focus_lock:bool,
    pressed_tab:bool,
    pressed_shift_tab:bool
}

impl Focus {
    pub fn focused(&self)->Option<Id>{
        self.id
    }

    pub fn begin_frame(&mut self ,  new_input:&RawInput) {
        self.id_prev_frame = self.id;

        if let Some(id) = self.id_next_frame.take() {
            self.id = Some(id);
        }

        self.pressed_tab = false;
        self.pressed_shift_tab = false;

        for event in &new_input.events {
            if matches!(
                event,
                Event::Key {
                    key:KeyCode::Escape,
                    pressed:true,
                    modifiers:_,
                }
            ){
                self.id = None;
                self.is_focus_lock = false;
                break;
            }

            if let Event::Key {
                key:KeyCode::Tab,
                pressed:true,
                modifiers,
            } = event {
                if !self.is_focus_lock {
                    if modifiers.shift {
                        self.pressed_shift_tab = true;
                    }else {
                        self.pressed_tab = true;
                    }
                }
            }
        }
    }

    pub fn end_frame(&mut self , used_id:IdMap<Rect>){
            // Allow calling `request_focus` one frame and not using it until next frame
        if let Some(id) = self.id {
            let recently_gained_focus = self.id_prev_frame != Some(id);

            if !recently_gained_focus && !used_id.contains_key(&id) {
                self.id = None
            }
        }
    }
}