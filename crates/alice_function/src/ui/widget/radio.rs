use alice_core::{math::Vector2f, color::{Rgba, Color}};

use crate::{geometry::rect::Rect, paint::shape::Shape, ui::{style::{StyleSheet, Spaces}, layer::LayerId, id::Id, store::Store, response::Response}};

use super::{label::Label, icon::Icon};


pub struct Radio {

    checked:bool,
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
    // 选中icon
    checked_icon:String,
    // 未选中icon
    uncheck_icon:String


}


// set props
impl Radio {
    pub fn new( checked:bool ,text:&str ) -> Self {
        Self { 
            checked,
            size: Vector2f::new(150.0,32.0), 
            pos: Vector2f::ZERO, 
            text: text.to_owned(), 
            background: Rgba::WHITE, 
            color: Rgba::BLACK,
            margin:Default::default() ,
            padding:Default::default(),
            checked_icon:String::from("assets/icon/checked.png"),
            uncheck_icon:String::from("assets/icon/unchecked.png")
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

impl super::Widget for Radio {
    fn ui(&mut self, ctx:&mut super::ui_context::UiContext) -> Response{

      
        let (id ,( x, y )) = ctx.allocate_virtual_space((self.size.x,self.size.y));
        if self.pos.x < x {
            self.pos.x = x;
        }

        if self.pos.y < y {
            self.pos.y = y
        }

 

        let rect = Rect::from_min_size(self.pos, self.size);

        let response = ctx.ctx.interact(id , rect);

        let (fill,icon_path) = if self.checked {
            (ctx.ctx.borrow().theme.success_color,&self.checked_icon)
        }else{
            (ctx.ctx.borrow().theme.error_color,&self.uncheck_icon)
        };

        {
            let shape = Shape::fill_rect( rect , fill, 0.0 );
            let painter = &mut ctx.ctx.borrow_mut().painter;
            painter.add_shape(LayerId::Document,shape);
        }



        Icon::new(icon_path).ui(ctx);
        Label::new(&self.text).ui(ctx);

  
        ctx.active_virtual_space((x + self.size.x,y));


        response
        // {

        //     // debug
        //     let rect = Rect::from_min_size(ctx.layout.pos, ctx.layout.used_space);

        //     let shape = Shape::fill_rect( rect , Rgba::MIDDLE_SPRING_GREEN, 0.0 );
        //     let painter = &mut ctx.ctx.borrow_mut().painter;
        //     painter.add_shape(LayerId::Document,shape);
        // }


    }
}