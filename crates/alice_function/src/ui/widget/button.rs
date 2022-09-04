use alice_core::{math::Vector2f, color::{Rgba, Color}};

use crate::{geometry::rect::Rect, paint::shape::Shape, ui::{style::{StyleSheet, Spaces, Position}, layer::LayerId}};

pub struct Button {
    // 大小
    size:Vector2f,
    // 位置
    pos: Vector2f,
    // 文案
    text:String,
    // 背景颜色
    background:Rgba,
    // 文本颜色
    color:Rgba,
    // 间距
    margin:Spaces,
    // 内边距
    padding:Spaces,



}


// set props
impl Button {
    pub fn new( text:&str ) -> Self {
        Self { 
            size: Vector2f::new(120.0,32.0), 
            pos: Vector2f::ZERO, 
            text: text.to_owned(), 
            background: Rgba::WHITE, 
            color: Rgba::BLACK,
            margin:Default::default() ,
            padding:Default::default(),
 
   
        }
    }

    pub fn set_style(mut self ,style:StyleSheet) -> Self {
        match style {
            StyleSheet::Background(bg)=> {
                self.background = bg;
            }
            StyleSheet::Color(color)=>{
                self.color = color;
            }
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

            StyleSheet::PaddingLeft(l)=>{
                self.padding.left = l;
            }
            StyleSheet::PaddingRight(r)=>{
                self.padding.right = r;
            }
            StyleSheet::PaddingTop(t)=>{
                self.padding.top = t;
            }
            StyleSheet::PaddingBottom(b)=>{
                self.padding.bottom = b;
            }

            StyleSheet::Position(p)=>{
                self.pos = p;
            }

          

            _ => {}
        };
        self
    }
}

impl super::Widget for Button {
    fn ui(&mut self, ctx:&mut super::ui_context::UiContext) {
        let ( x, y ) = ctx.layout.allocate((self.size.x,self.size.y));
        if self.pos.x < x {
            self.pos.x = x;
        }

        if self.pos.y < y {
            self.pos.y = y
        }

       

        let rect = Rect::from_min_size(self.pos, self.size);
        let shape = Shape::fill_rect( rect , Color::BLUE, 0.0 );
        let painter = &mut ctx.ctx.borrow_mut().painter;
        painter.add_shape(LayerId::Document,shape);

    }
}