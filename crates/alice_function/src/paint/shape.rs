use alice_core::{color::{Color, Rgba}, math::Vector2f};

use crate::geometry::rect::Rect;

use super::stroke::Stroke;



#[derive(Debug,Default)]
pub struct Round {
   pub nw:f32,
   pub ne:f32,
   pub sw:f32,
   pub se:f32
}


impl Round {
    pub fn is_empty(&self) -> bool {
        self.nw == 0.0 && 
        self.ne == 0.0 && 
        self.sw == 0.0 && 
        self.se == 0.0
    }
}

impl From<f32> for Round {
    fn from(round: f32) -> Self {
        Self {
            nw:round,
            ne:round,
            sw:round,
            se:round,
        }
    }
}

pub enum Shape {
    Null,
    Children(Vec<Shape>),
    Path(PathShape),
    Rect(RectShape),
    Circle(CircleShape),
    Text(TextShape),
    Icon(IconShape)
}


impl Shape {
    pub fn stroke_rect(  rect:Rect , stroke:Stroke , round:impl Into<Round> ) -> Self {
        Shape::Rect(RectShape::stroke(rect, stroke, round))
    }
    pub fn fill_rect( rect:Rect , fill:impl Into<Color> , round:impl Into<Round>  ) -> Self {
        Shape::Rect(RectShape::fill(rect, fill, round))
    }

    pub fn draw_text(text:&str , color:Rgba , pos:Vector2f) -> Self {
        Self::Text(TextShape::draw(text, color,pos))
    }

    pub fn draw_icon(path:&str , size:Vector2f,pos:Vector2f) -> Self {
        Self::Icon(IconShape::draw(path, size, pos))
    }
}

pub struct PathShape {

}


pub struct RectShape {
   pub rect:Rect,
   pub stroke:Stroke,
   pub fill:Color,
   pub round:Round
}

impl RectShape {
    pub fn stroke( rect:Rect , stroke:Stroke , round:impl Into<Round> ) -> Self {
        Self { rect, stroke, fill: Color::TRANSPARENT, round:round.into() }
    }

    pub fn fill( rect:Rect , fill:impl Into<Color> , round:impl Into<Round>  ) -> Self {
        Self { rect, stroke:Stroke::default(), fill:fill.into(),  round:round.into() }
    }
}

pub struct CircleShape {

}


pub struct TextShape {
    pub text:   String,
    pub color:  Rgba,
    pub pos:    Vector2f
}


impl TextShape {
    pub fn draw( text:&str  , color:Rgba , pos:Vector2f) -> Self {
        Self { text: text.to_owned(), color , pos }
    }
}


pub struct IconShape {
    pub path:   String,
    pub size:   Vector2f,
    pub pos:    Vector2f
}


impl IconShape {
    pub fn draw( path:&str  , size:Vector2f , pos:Vector2f) -> Self {
        Self { path: path.to_owned(), size , pos }
    }
}