use alice_core::color::Color;

#[derive(Debug , Clone, Copy)]
pub struct Stroke{
    pub width:f32,
    pub color:Color
}

impl std::default::Default for Stroke {
    fn default() -> Self {
        Self { width:1.0 , color: Color::WHITE }
    }
}

impl Stroke {
    pub const PLACEHOLDER:Stroke = Stroke { width:0.0 , color:Color::TRANSPARENT };
}

