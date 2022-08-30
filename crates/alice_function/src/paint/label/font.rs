
use ab_glyph::{ScaleFont};
use alice_core::math::{Vector2u, Vector2f};
use image::{GenericImage, GenericImageView};

use super::texture_atlas::FontAtlas;


#[derive(Debug,Default)]
pub struct UvRect {
    pub offset:Vector2f,
    pub size:Vector2f,  // 屏幕上显示的大小

    pub min:[u16;2], // 左上角
    pub max:[u16;2], // 右下角
}


pub struct GlyphInfo {
    id:ab_glyph::GlyphId,

    advance_width:f32,

    uv:UvRect
}


// 存储一种字体, 大小要固定
pub struct Font {
    font: ab_glyph::FontArc,
    font_atlas: FontAtlas,
    
}



impl Font {
    pub fn new(path:&str) -> Self {
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("../../../../../assets/font/MONACO.TTF")).expect("load font failed!");

        let font_atlas = FontAtlas::new(1024, 64);

        Self { font, font_atlas }
    }

    pub fn glyph_info(&mut self , chr:char){
        use ab_glyph::Font as _;
        let id = self.font.glyph_id(chr);
        allocate_glyph(&mut self.font_atlas, &mut self.font, id, 24.0);
    }

    pub fn pre_common_char(&mut self){
        const START_ASCII:u8 = 33;
        const END_ASCII:u8 = 126;

        for c in (START_ASCII..=END_ASCII).map(|c|c as char) {
            self.glyph_info(c);
        }

        self.font_atlas.image.save("test.png");
    }

}



pub fn allocate_glyph( atlas:&mut FontAtlas , font:&mut ab_glyph::FontArc , glyph_id:ab_glyph::GlyphId , font_size:f32 ) -> GlyphInfo {
    use ab_glyph::Font as _;
    let glyph = glyph_id.with_scale_and_position(font_size, ab_glyph::point(0.0, 0.0));
   
    let uv = font.outline_glyph(glyph).map(|glyph|{
        let bb = glyph.px_bounds();
        let glyph_width = bb.width() as usize;
        let glyph_height = bb.height() as usize;
    
    
        if glyph_width == 0 || glyph_height == 0 {
            UvRect::default()
        }else {
            let ( pos , image ) = atlas.allocate((glyph_width , glyph_height));

            glyph.draw(|x,y,c|{          
                let x = pos.0 as u32 + x ;
                let y = pos.1 as u32 + y;
                 // Offset the position by the glyph bounding box
                let px = image.get_pixel(x, y);
                 // Turn the coverage into an alpha value (blended with any previous)
                let px = image::Rgba([
                    255,
                    0,
                    0,
                    px.0[3].saturating_add((c * 255.0) as u8),
                ]);
                image.put_pixel(x, y, px);
                // image.put_pixel(x, y + font_size as u32, px);
                    
               
            });
    
    
            let offset = Vector2f::new(bb.min.x,font_size + bb.min.y);
            let size = Vector2f::new(glyph_width as f32,glyph_height as f32);
            UvRect {
                offset,
                size,
                min: [pos.0 as u16,pos.1 as u16],
                max:[(pos.0 + glyph_width) as u16 , (pos.1 + glyph_height) as u16],
            }
        }   

     
        
    }).unwrap_or_default();

    let advance_width = font.as_scaled(font_size).h_advance(glyph_id);
    GlyphInfo { id:glyph_id, advance_width, uv }


}
