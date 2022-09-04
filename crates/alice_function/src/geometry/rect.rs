use alice_core::math::Vector2f;



#[derive(Debug,Clone,PartialEq,Copy,Default)]
pub struct Rect {
    pub min_point:  Vector2f,
    pub max_point:   Vector2f,
}


impl Rect {
    pub const NONE:Rect = Rect {
        max_point:Vector2f {x:f32::NEG_INFINITY ,y: f32::NEG_INFINITY},
        min_point:Vector2f{x:f32::INFINITY,y:f32::INFINITY}
    };
    pub fn new(min_point:Vector2f,max_point:Vector2f) -> Self {
        Rect {
            min_point,
            max_point,
        }
    }

    pub fn from_min_size(min_point:Vector2f , size:Vector2f) -> Self {
        Rect {
            min_point,
            max_point:min_point + size
        }
    }

 
    pub fn width(&self) -> f32 {
        self.max_point.x - self.min_point.x
    }

    pub fn height(&self) -> f32 {
        self.max_point.y - self.min_point.y
    }

    pub fn half_width(&self) -> f32 {
        self.width() / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.height() / 2.0
    }

    pub fn center(&self) -> Vector2f {
        Vector2f::new(self.min_point.x + self.half_width(),self.min_point.y + self.half_height())
    }

    pub fn container(&self , point:Vector2f) -> bool {
        self.min_point.x <= point.x && self.max_point.x >= point.x &&
        self.min_point.y <= point.y && self.max_point.y >= point.y 
    }

    pub fn extend_include_point(&mut self, point:Vector2f) {
        if point.x < self.min_point.x {
            self.min_point.x = point.x;
        }
        if point.y < self.min_point.y {
            self.min_point.y = point.y;
        }
        if point.x > self.max_point.x {
            self.max_point.x = point.x;
        }
        if point.y > self.max_point.y {
            self.max_point.y = point.y;
        }
    }

    pub fn extend_base(&mut self, size:Vector2f) {
        self.min_point.x -= size.x;
        self.min_point.y -= size.y;
        self.max_point.x += size.x;
        self.max_point.y += size.y;
    }

  

    pub fn extend(&mut self , size:Vector2f){
        self.max_point.x += size.x;
        self.max_point.y += size.y;
    }

    pub fn extend_x(&mut self , size:f32) {
        self.max_point.x += size;
    }


    pub fn extend_y(&mut self , size:f32) {
        self.max_point.y += size;
    }

    pub fn shrink_base(&mut self, size:Vector2f) {
        self.min_point.x += size.x;
        self.min_point.y += size.y;
        self.max_point.x -= size.x;
        self.max_point.y -= size.y;

        if self.min_point.x > self.max_point.x {
            let temp = self.min_point.x;
            self.min_point.x = self.max_point.x;
            self.max_point.x = temp;
        }
        if self.min_point.y > self.max_point.y {
            let temp = self.min_point.y;
            self.min_point.y = self.max_point.y;
            self.max_point.y = temp;
        }
    }

    pub fn shrink(&mut self, size:Vector2f) {
        self.max_point.x -= size.x;
        self.max_point.y -= size.y;
    }

    pub fn intersects(&self, other:&Rect) -> bool {
        self.min_point.x < other.max_point.x && self.max_point.x > other.min_point.x && self.min_point.y < other.max_point.y && self.max_point.y > other.min_point.y
    }
    #[inline(always)]
    pub fn top(&self)->f32 {
        self.min_point.y
    }

    #[inline(always)]
    pub fn right(&self)->f32 {
        self.max_point.x
    }
    #[inline(always)]
    pub fn bottom(&self)->f32 {
        self.max_point.y
    }
    #[inline(always)]
    pub fn left(&self)->f32 {
        self.min_point.x
    }

    #[inline(always)]
    pub fn size(&self) -> Vector2f {
        Vector2f::new(self.width(),self.height())
    }

    #[inline(always)]
    pub fn translate(&self , xy:Vector2f) -> Self {
        Rect::from_min_size(self.min_point + xy, self.size())
    } 

    #[inline(always)]
    pub fn left_top(&self)->Vector2f {
        self.min_point
    }
    #[inline(always)]
    pub fn right_top(&self)->Vector2f {
        Vector2f::new(self.max_point.x,self.min_point.y)
    }

    #[inline(always)]
    pub fn left_bottom(&self)->Vector2f {
        Vector2f::new(self.min_point.x,self.max_point.y)
    }
    #[inline(always)]
    pub fn right_bottom(&self)->Vector2f {
        self.max_point
    }

}