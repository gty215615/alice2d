use alice_core::{math::Vector2f, color::{Rgba, Color}};

use crate::{geometry::rect::Rect, paint::shape::Shape, ui::{style::{StyleSheet, Spaces, Position}, layer::LayerId, response::{self, Response}}};

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
    fn ui(&mut self, ctx:&mut super::ui_context::UiContext)-> Response {
     
 
        let mut advance_width = 0.0;
        let mut str = String::new();
        let mut height = 0.0;
            
       {
            let painter = &mut ctx.ctx.borrow_mut().painter;

            for chr in self.text.chars() {
                let info = painter.font.glyph_info(chr);
                advance_width += info.advance_width;
                height = info.uv.size.y;
                if advance_width < self.size.x {
                    str.push(chr)
                }else{
                    break;
                }
            }
       }

   
        let (id , ( x, y )) = ctx.allocate((advance_width,32.0));

        
        if self.pos.x < x {
            self.pos.x = x;
        }

        if self.pos.y < y {
            self.pos.y = y
        }


        let rect = Rect::from_min_size(self.pos, Vector2f::new(advance_width,32.0));

        let response = ctx.ctx.interact(id , rect);

        let painter = &mut ctx.ctx.borrow_mut().painter;

        let text = Shape::draw_text(&str, self.color, self.pos);
        painter.add_shape(LayerId::Document,text);

        response
    }
}