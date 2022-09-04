use alice_core::math::Vector2f;

use crate::input::{input_state::POINTER_BUTTON_NUM, MouseButton};

use super::{context::Context, id::Id};



pub struct Response {
    pub ctx:Context , 
    pub id:Id,
    pub hovered:bool,
    pub clicked:[bool;POINTER_BUTTON_NUM],
    pub double_clicked:[bool;POINTER_BUTTON_NUM],
    pub triple_clicked:[bool;POINTER_BUTTON_NUM],
     /// Where the pointer (mouse/touch) were when when this widget was clicked or dragged.
    /// `None` if the widget is not being interacted with.
    pub is_pointer_button_down:bool,
    #[doc(hidden)]
    pub interact_pointer_pos: Option<Vector2f>,
    #[doc(hidden)]
    pub changed: bool,
}


impl Response {
    pub fn clicked(&self) -> bool {
        self.clicked[MouseButton::Left as usize]
    }

    pub fn hovered(&self) -> bool {
        self.hovered
    }

}