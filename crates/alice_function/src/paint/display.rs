use alice_core::math::{Vector2f};

use crate::geometry::rect::Rect;

pub struct Layout {
    pub used_space:Vector2f,
    pub row_height:f32,
    pub size:      Vector2f,
    pub pos:       Vector2f
}


impl Layout {
    pub fn new( pos:Vector2f , w:f32 , h:f32 ) -> Self {
        Self {pos , used_space: Vector2f::new(0.0, 0.0), row_height: 0.0, size: Vector2f::new(w,h) }
    }

    pub fn allocate(&mut self , (w,h):(f32,f32) ) -> ( f32 , f32 ) {
         
        let space = self.pos + self.used_space;
        let mut space = (space.x , space.y);

        if w + self.used_space.x > self.size.x {
            // 换行
            self.used_space.x = self.pos.x;
            self.used_space.y += self.row_height;
            self.row_height = 0.0;


            space.0 = self.used_space.x;
            space.1 = self.used_space.y;

          
        }
        self.used_space.x += w;
        self.row_height = if self.row_height < h { h } else { self.row_height };

        space
        
    }

    pub fn clip_rect(&self)->Rect{
        Rect::from_min_size(self.pos, Vector2f::new(self.size.x,self.used_space.y + self.row_height))
    }

    pub fn clear(&mut self){
      self.used_space = Vector2f::ZERO;
      self.row_height = 0.0;
    }
}