use alice_core::{color::Rgba, math::Vector2f};


pub enum Overflow {
    Hidden,
    Scroll
}





pub enum DisplayType {
    Inline,
    InlineBlock,
    Block
}


pub enum Position {
    Static,
    Relative,
    Fixed,
    Absolute
}


pub struct Spaces {
   pub left:f32,
   pub right:f32,
   pub top:f32,
   pub bottom:f32
}

impl Default for Spaces {
    fn default() -> Self {
        Self { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 }
    }
}


pub enum StyleSheet {
    Width(f32),
    Height(f32),
    FontSize(f32) , // 1 unit  
    PaddingLeft(f32),
    PaddingBottom(f32),
    PaddingRight(f32),
    PaddingTop(f32),
    MarginLeft(f32),
    MarginTop(f32),
    MarginRight(f32),
    MarginBottom(f32),
    Color(Rgba),  // font color
    Background(Rgba),
    Overflow(Overflow),
    Display(DisplayType),
    Position(Vector2f), // relative parent node
   
}

