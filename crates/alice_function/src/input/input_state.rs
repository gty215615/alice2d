use std::collections::HashSet;

use alice_core::math::Vector2f;

use crate::geometry::rect::Rect;

use super::{raw_input::RawInput, MouseButton, Modifiers, KeyCode, event::Event};


// 点击时鼠标位置最大的移动距离, 超过就会自动取消点击事件
const MAX_CLICK_DIST:f32 = 6.0;
// 点击时持续的是时间, 超过就会自动取消点击事件
const MAX_CLICK_DURATION:f32 = 0.6;
// 双击间隔不能超过 0.3
const MAX_DOUBLE_CLICK_DELAY:f32 = 0.3;

pub const POINTER_BUTTON_NUM:usize = 3;

pub struct InputContext {
    pub raw:RawInput,
    pub pointer:PointerState,
    pub scroll_delta:Vector2f,
    zoom_factor_delta:f32,
    pub screen_rect:Rect,
    pub time:f64,
    pub unstable_dt:f32,
    pub predicted_dt:f32,
    pub modifiers:Modifiers,
    pub keys_down:HashSet<KeyCode>,
    pub events:Vec<Event>
}


impl Default for InputContext {
    fn default() -> Self {
        Self { 
            raw: Default::default(), 
            pointer: Default::default(), 
            scroll_delta: Default::default(), 
            zoom_factor_delta: 1.0, 
            screen_rect: Rect::from_min_size(Default::default(), Vector2f::new(1280.0, 760.0)), 
            time: 0.0, 
            unstable_dt: 1.0 / 6.0, 
            predicted_dt: 1.0 / 6.0, 
            modifiers: Default::default(), 
            keys_down: Default::default(), 
            events: Default::default()
        }
    }
}

impl InputContext {
    pub fn begin_time(mut self , new:RawInput) -> InputContext {
        let time = new.time.unwrap_or(self.time + new.predicted_dt as f64);
        let unstable_dt = (time - self.time) as f32;

        let screen_rect = new.screen_rect.unwrap_or(self.screen_rect);

        let pointer = self.pointer.begin_time(time, &new);

        let mut keys_down = self.keys_down;
        let mut scroll_delta = Vector2f::ZERO;
        let mut zoom_factor_delta = 1.0;

        for event in &new.events {
            match event {
                Event::Key { key, pressed, modifiers } => {
                    if *pressed {
                        keys_down.insert(*key);
                    }else {
                        keys_down.remove(key);
                    }
                }
                Event::Scroll(delta) => {
                    scroll_delta += *delta;
                }
                Event::Zoom(factor) => {
                    zoom_factor_delta *= *factor;
                }
                _ => {}
            }
        }

        InputContext { 
            
            pointer, 
            scroll_delta, 
            zoom_factor_delta, 
            screen_rect, 
            time, 
            unstable_dt, 
            predicted_dt:new.predicted_dt, 
            modifiers: new.modifiers, 
            keys_down, 
            events: new.events.clone(), // TODO remove clone and use raw.events
            raw: new, 
        }



    }
}

// Mouse State 

pub struct PointerState {
    // 最新一帧的时间
    time:f64,
    // 最新的位置
    latest_pos:Option<Vector2f>,
    // 与上一帧位置的差
    delta:Vector2f,
    // 鼠标的速度
    // velocity:Vector2f,
    // 鼠标最近的移动
    // History

    down:[bool;POINTER_BUTTON_NUM],

    // 鼠标点击的点
    press_origin:Option<Vector2f>,
    // 鼠标点击的时间
    press_start_time:Option<f64>,

    has_move_too_much_for_once_click:bool,
    // 上次点击时间
    last_click_time:f64,
    // 上上次点击时间
    last_last_click_time:f64,

   pub pointer_events:Vec<PointerEvent>

}

impl Default for PointerState {
    fn default() -> Self {
        Self { 
            time: -f64::INFINITY,
            latest_pos: None,
            delta: Vector2f::ZERO, 
            down: Default::default(),
            press_origin: None, 
            press_start_time: None, 
            has_move_too_much_for_once_click: false, 
            last_click_time: f64::NEG_INFINITY, 
            last_last_click_time: f64::NEG_INFINITY,  
            pointer_events: Vec::new() 
        }
    }
}

impl PointerState {
    #[must_use]
    pub(crate) fn begin_time(mut self , time:f64 , new:&RawInput) -> PointerState {
        self.time = time;

        self.pointer_events.clear();

        let old_pos = self.latest_pos;

        for event in &new.events {
            match event {
                Event::PointerMoved(pos) => {
                    let pos = *pos;
                    self.latest_pos = Some(pos);

                    if let Some(press_origin) = self.press_origin {
                        self.has_move_too_much_for_once_click |= 
                            press_origin.distance(pos) > MAX_CLICK_DIST
                    }

                    self.pointer_events.push(PointerEvent::Moved(pos));
                }
                Event::PointerButton { 
                    pos, 
                    button, 
                    pressed, 
                    modifiers 
                } => {
                    let pos = *pos;
                    let button = *button;
                    let pressed = *pressed;
                    let modifiers = *modifiers;

                    self.latest_pos = Some(pos);

                    if pressed {
                        self.press_origin = Some(pos);
                        self.press_start_time = Some(time);
                        self.has_move_too_much_for_once_click = false;
                        self.pointer_events.push( PointerEvent::Pressed(pos) );
                    }else{
                        let clicked = self.may_be_any_button_be_click();

                        let click = if clicked {
                            let double_click = ((time - self.last_click_time) as f32) < MAX_DOUBLE_CLICK_DELAY;

                            let triple_click = ((time - self.last_last_click_time) as f32)  < (MAX_DOUBLE_CLICK_DELAY * 2.0);

                            let count = if triple_click {
                                3
                            }else if double_click {
                                2
                            }else {
                                1
                            };

                            self.last_last_click_time = self.last_click_time;
                            self.last_click_time = self.time;

                            Some(Click {
                                pos,
                                button,
                                count,
                                modifiers,
                            })
                        }else {
                            None
                        };

                        self.pointer_events.push(PointerEvent::Released(click));

                        self.press_origin = None;
                        self.press_start_time = None;
                    }

                    self.down[button as usize] = pressed;
                }
                Event::PointerGone => {
                    self.latest_pos = None
                }
                _ => {}
            }
        }


        self.delta = if let (Some(old_pos) , Some(new_pos)) = ( old_pos , self.latest_pos ) {
            new_pos - old_pos
        }else {
            Vector2f::ZERO
        };

        // update history
        self
    }

    #[inline(always)]
    pub fn delta(&self) -> Vector2f {
        self.delta
    }

    #[inline(always)]
    pub fn press_origin(&self) -> Option<Vector2f> {
        self.press_origin
    }

        /// When did the current click/drag originate?
    /// `None` if no mouse button is down.
    #[inline(always)]
    pub fn press_start_time(&self) -> Option<f64> {
        self.press_start_time
    }

    /// Latest reported pointer position.
    /// When tapping a touch screen, this will be `None`.
    #[inline(always)]
    pub(crate) fn latest_pos(&self) -> Option<Vector2f> {
        self.latest_pos
    }

    /// If it is a good idea to show a tooltip, where is pointer?
    #[inline(always)]
    pub fn hover_pos(&self) -> Option<Vector2f> {
        self.latest_pos
    }
    
     /// Do we have a pointer?
    ///
    /// `false` if the mouse is not over the egui area, or if no touches are down on touch screens.
    #[inline(always)]
    pub fn has_pointer(&self) -> bool {
        self.latest_pos.is_some()
    }

        /// Was any pointer button pressed (`!down -> down`) this frame?
    /// This can sometimes return `true` even if `any_down() == false`
    /// because a press can be shorted than one frame.
    pub fn any_pressed(&self) -> bool {
        self.pointer_events.iter().any(|event| event.is_press())
    }



    /// Was any pointer button released (`down -> !down`) this frame?
    pub fn any_released(&self) -> bool {
        self.pointer_events.iter().any(|event| event.is_release())
    }

    /// Is any pointer button currently down?
    pub fn any_down(&self) -> bool {
        self.down.iter().any(|&down| down)
    }

    /// Were there any type of click this frame?
    pub fn any_click(&self) -> bool {
        self.pointer_events.iter().any(|event| event.is_click())
    }


    /// Is this button currently down?
    #[inline(always)]
    pub fn button_down(&self, button: MouseButton) -> bool {
        self.down[button as usize]
    }


    #[inline(always)]
    pub fn may_be_any_down(&self) -> bool {
        self.down.iter().any(|&down|down)
    }
    #[inline(always)]
    pub fn may_be_any_button_be_click(&self) -> bool {
        if !self.may_be_any_down() {
            return false;
        }

        // 如果移动距离太远,直接取消点击时间
        if self.has_move_too_much_for_once_click {
            return false
        }

        if let Some(press_start_time) = self.press_start_time {
            if ((self.time - press_start_time) as f32) > MAX_CLICK_DURATION {
                return false
            }
        }

        true
    }
}


#[derive(Debug,Clone , PartialEq)]
pub struct Click {
    pub pos:Vector2f,
    pub button:MouseButton,
    pub count:u32,
    pub modifiers:Modifiers
}   

impl Click {
    pub fn is_double(&self) -> bool {
        self.count == 2
    }

    pub fn is_triple(&self) -> bool {
        self.count == 3
    }
}

pub enum PointerEvent {
    Moved(Vector2f),
    Pressed(Vector2f),
    Released(Option<Click>)
}

impl PointerEvent {
    pub fn is_press(&self) -> bool {
        matches!(self,PointerEvent::Pressed(_))
    }
    pub fn is_release(&self) -> bool {
        matches!(self,PointerEvent::Released(_))
    }
    pub fn is_click(&self) -> bool {
        matches!(self,PointerEvent::Released(Some(_)))
    }
}