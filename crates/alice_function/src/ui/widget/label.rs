use alice_core::{math::Vector2f, color::{Rgba, Color}};

use crate::{geometry::rect::Rect, paint::shape::Shape, ui::{style::{StyleSheet, Spaces, Position}, layer::LayerId}};

pub struct Label {
    // 大小
    size:Vector2f,
    // 位置
    pos: Vector2f,
    // 文案
    text:String,
    // 文本颜色
    color:Rgba,
}


// set props
impl Label {
    pub fn new( text:&str ) -> Self {
        Self { 
            size: Vector2f::new(f32::MAX,f32::MAX), 
            pos: Vector2f::ZERO, 
            text: text.to_owned(), 
   
            color: Rgba::BLACK,
        
        }
    }

    pub fn set_style(mut self ,style:StyleSheet) -> Self {
        match style {

            StyleSheet::Color(color)=>{
                self.color = color;
            }
            StyleSheet::Width(w)=>{
                self.size.x = w;
            }
            StyleSheet::Height(h)=>{
                self.size.y = h;
            }
            StyleSheet::Position(p)=>{
                self.pos = p;
            }
         
            _ => {}
        };
        self
    }
}

impl super::Widget for Label {
    fn ui(&mut self, ctx:&mut super::ui_context::UiContext) {
     

        let painter = &mut ctx.ctx.borrow_mut().painter;

        let mut advance_width = 0.0;
        let mut str = String::new();
        for chr in self.text.chars() {
            advance_width += painter.font.glyph_info(chr).advance_width;
            if advance_width < self.size.x {
                str.push(chr)
            }else{
                break;
            }
        }

    


        let text = Shape::draw_text(&str, self.color, self.pos);
        painter.add_shape(LayerId::Document,text);

    }
}