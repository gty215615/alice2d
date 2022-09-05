use std::cell::RefCell;

use alice_core::math::{Vector2u, Vector2};






pub struct FontAtlas {
    size:       Vector2<usize>,
    used_place: Vector2<usize>,
    pub(crate) image:      image::DynamicImage,
    row_height: usize,
}

impl FontAtlas {
    pub fn new(w:usize , h:usize) -> Self {

        let mut image_buffer:image::RgbaImage = image::ImageBuffer::new(w as u32,  h as u32);
        
        image_buffer.put_pixel(0, 0, image::Rgba([255,255,255,255]));

        let image = image::DynamicImage::ImageRgba8(image_buffer);

        Self { size: Vector2::new(w,h), used_place: Vector2::new(1,0), image, row_height: 0 }
    }

    pub fn used_space(&self) -> Vector2<usize>{
        self.used_place
    }

    pub fn allocate(&mut self , (w,h):(usize,usize) ) -> ( (usize,usize) , &mut image::DynamicImage ) {
        
        let mut space = self.used_place;

     
        if w + self.used_place.x > self.size.x {
            // 换行
            self.used_place.x = 0;
            self.used_place.y += self.row_height;
            self.row_height = 0;
            space = self.used_place;
        }
        self.used_place.x += w;
             
        self.row_height = if self.row_height < h { h } else { self.row_height };

        // TODO used_place.y > size.y ???
        
        (( space.x , space.y ) , &mut self.image)
    }
}





