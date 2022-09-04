
use std::collections::HashMap;

use ab_glyph::{ScaleFont};
use alice_core::math::{Vector2u, Vector2f};
use image::{GenericImage, GenericImageView};

use super::texture_atlas::FontAtlas;


#[derive(Debug,Default , Clone, Copy)]
pub struct UvRect {
    pub offset:Vector2f,
    pub size:Vector2f,  // 屏幕上显示的大小

    pub min:[u16;2], // 左上角
    pub max:[u16;2], // 右下角
}

pub const DEFAULT_FONT_SIZE:f32 = 24.0;

#[derive(Clone, Copy)]
pub struct GlyphInfo {
    id:ab_glyph::GlyphId,

   pub advance_width:f32,

   pub uv:UvRect
}

#[derive(Clone, Copy)]
pub struct IconInfo {
   pub uv:UvRect
}



// 存储一种字体, 大小要固定
pub struct Font {
    font: ab_glyph::FontArc,
    font_atlas: FontAtlas,
    font_cache: HashMap<char , GlyphInfo>,
    icon_cache: HashMap<String , IconInfo>,
}



impl Font {
    pub fn new(path:&str) -> Self {
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("../../../../../assets/font/MONACO.TTF")).expect("load font failed!");

        let font_atlas = FontAtlas::new(1024, 64);

        Self { font, font_atlas , font_cache:HashMap::new()  , icon_cache:HashMap::new()}
    }

    pub fn glyph_info(&mut self , chr:char) -> GlyphInfo{
        use ab_glyph::Font as _;

        if self.font_cache.contains_key(&chr){
            return *self.font_cache.get(&chr).unwrap();
        }

        let id = self.font.glyph_id(chr);
        let glyph_info = allocate_glyph(&mut self.font_atlas, &mut self.font, id, DEFAULT_FONT_SIZE);

        self.font_cache.insert(chr, glyph_info);

        glyph_info
    }

    pub fn pre_common_char(&mut self){
        const START_ASCII:u8 = 33;
        const END_ASCII:u8 = 126;

        for c in (START_ASCII..=END_ASCII).map(|c|c as char) {
            self.glyph_info(c);
        }

        self.compose_icon("assets/icon/checked.png");
        self.compose_icon("assets/icon/unchecked.png");
        self.font_atlas.image.save("test.png");
    }


    pub fn query_icon(&mut self , path:&str) -> IconInfo {
        if self.icon_cache.contains_key(&path.to_owned()) {
             *self.icon_cache.get(path).unwrap()
        }else{
            self.compose_icon(path)
        }
    }

    pub fn compose_icon(&mut self , path:&str) -> IconInfo {
        use image::io::Reader as ImageReader;
        let img = ImageReader::open(path).expect(&format!("load image {} failed",path)).decode().expect(&format!("decode image {} failed",path));
        let size = img.dimensions();
      
        let (pos , atlas) = self.font_atlas.allocate((size.0 as usize , size.1 as usize));
        
        atlas.copy_from(&img, pos.0 as u32, pos.1 as u32);


      
        let uv = UvRect {
            offset:Vector2f::ZERO,
            size:Vector2f::new(size.0 as f32,size.1 as f32),
            min: [pos.0 as u16,pos.1 as u16],
            max:[(pos.0 + size.0 as usize) as u16 , (pos.1 + size.1 as usize) as u16],
        };

        let icon_info = IconInfo {
            uv
        };

        self.icon_cache.insert(path.to_owned(), icon_info);
        icon_info
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
              
                image.put_pixel(x, y, image::Rgba([
                    255,
                    255,
                    255,
                    (c * 255.0) as u8
                ]));
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
