use alice_core::math::{Vector2f};

use crate::geometry::rect::Rect;

pub struct Layout {
    pub used_space:Vector2f,
    pub row_height:f32,
    pub size:      Vector2f,
    pub pos:       Vector2f,
}


impl Layout {
    pub fn new( pos:Vector2f , w:f32 , h:f32 ) -> Self {
        Self {
            pos , 
            used_space: Vector2f::new(0.0, pos.y), 
            row_height: 0.0, 
            size: Vector2f::new(w,h) 
        }
    }

    pub fn allocate(&mut self , (w,h):(f32,f32) ) -> ( f32 , f32 ) {
         
        let mut space = self.pos + self.used_space;
      
        if w + self.used_space.x  > self.size.x {
            // 换行
            self.used_space.x = 0.0;
            self.used_space.y += self.row_height;
            self.row_height = 0.0;

            space = self.pos + self.used_space;
        }
        self.used_space.x += w;
        self.row_height = if self.row_height < h { h } else { self.row_height };

       (space.x , space.y)

        
    }

    #[must_use = "if use this and then must be use active_virtual_space"]
    pub fn allocate_virtual_space(&mut self ,  (w,h):(f32,f32) ) -> ( f32 , f32 ) {
        let mut space = self.pos + self.used_space;
      
        if w + self.used_space.x > self.size.x {
            // 换行
            self.used_space.x = 0.0;
            self.used_space.y += self.row_height;
            self.row_height = 0.0;

            space = self.pos + self.used_space;
        }
 

        (space.x  , space.y)

    }

    pub fn active_virtual_space(&mut self , (x,y):(f32,f32)) {
        
        self.used_space = Vector2f::new( x  , y ) - self.pos;

    }

    pub fn clip_rect(&self)->Rect{
        Rect::from_min_size(self.pos, Vector2f::new(self.size.x,self.used_space.y + self.row_height))
    }

    pub fn clear(&mut self){
      self.used_space = Vector2f::ZERO;
      self.row_height = 0.0;
    }

    pub fn force_same_line(&mut self){
        self.used_space.x = 0.0;
        self.used_space.y += self.row_height;
    }
}