use alice_core::{math::Vector2f, color::{Rgba, Color}};

use crate::{geometry::rect::Rect, paint::shape::Shape, ui::{style::{StyleSheet, Spaces, Position}, layer::LayerId}};

pub struct Icon {
    // 大小
    size:Vector2f,
    // 位置
    pos: Vector2f,
    // 间距
    margin:Spaces,
    // url
    path:String
}


// set props
impl Icon {
    pub fn new( path:&str ) -> Self {
        Self { 
            size: Vector2f::new(32.0,32.0), 
            pos: Vector2f::ZERO, 
          
            margin:Default::default() ,
            path:path.to_owned()
 
   
        }
    }

    pub fn set_style(mut self ,style:StyleSheet) -> Self {
        match style {
         
            StyleSheet::Width(w)=>{
                self.size.x = w;
            }
            StyleSheet::Height(h)=>{
                self.size.y = h;
            }
            StyleSheet::MarginLeft(l)=>{
                self.margin.left = l;
            }
            StyleSheet::MarginRight(r)=>{
                self.margin.right = r;
            }
            StyleSheet::MarginTop(t)=>{
                self.margin.top = t;
            }
            StyleSheet::MarginBottom(b)=>{
                self.margin.bottom = b;
            }

         
            StyleSheet::Position(p)=>{
                self.pos = p;
            }

          

            _ => {}
        };
        self
    }
}

impl super::Widget for Icon {
    fn ui(&mut self, ctx:&mut super::ui_context::UiContext) {
        
       


        let shape = Shape::draw_icon(&self.path, self.size, self.pos);
        let painter = &mut ctx.ctx.borrow_mut().painter;
        painter.add_shape(LayerId::Document,shape);

    }
}