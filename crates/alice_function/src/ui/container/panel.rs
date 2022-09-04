use alice_core::{math::{Vector2f}, color::{ Rgba}};

use crate::{paint::{shape::Shape}, geometry::rect::Rect, ui::{widget::ui_context::UiContext, context::Context, style::{StyleSheet, Spaces, Position}, layer::LayerId, id::Id}};



pub struct Panel {
    pub pos:Vector2f,
    pub size:Vector2f,
    pub background:Rgba,
    // 文本颜色
    color:Rgba,
     // 间距
     margin:Spaces,
     // 内边距
     padding:Spaces,

}


/// props 
/// 
impl Panel {
    pub fn default() -> Self {
        Self {
            pos:                Default::default(),
            size:               Default::default(),
            background:         Rgba::BLACK,
            color:              Rgba::BLACK,
            margin:             Default::default() ,
            padding:            Default::default(),
           
   
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



impl super::Container for Panel {
    fn show(&mut self, ctx:&mut Context , render_children:impl FnOnce(&mut UiContext)  ) {

        let w = self.size.x  + self.margin.left +  self.margin.right ;
        let h = self.size.y  + self.margin.top +  self.margin.bottom;

        let ( x, y ) = ctx.borrow_mut().layout.allocate((w,h));
        let x = x + self.margin.left;
        let y = y + self.margin.top;
        if self.pos.x < x {
            self.pos.x = x;
        }

        if self.pos.y < y {
            self.pos.y = y
        }

        


        let rect = Rect::from_min_size(self.pos, self.size );


        // let response = ctx.interact(Id::new("Panel"),rect);

        let fill = ctx.borrow().theme.primary_color_hover ;

        let shape = Shape::fill_rect( rect , fill, 0.0 );
  
        ctx.borrow_mut().painter.add_shape( LayerId::Document , shape);
   
        let mut ui = UiContext::new(self.pos + Vector2f::new( self.padding.left,self.padding.top ), &self.size,ctx.clone());
        
        render_children(&mut ui);
        
       
    
    }
}