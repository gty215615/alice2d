use alice_core::{math::Vector2f, color::{Color, self}};


#[derive(Debug,Clone, Copy)]
pub struct Vertex {
   pub pos:    Vector2f,
   pub uv:     Vector2f,
   pub color:  Color
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}


#[derive(Debug,Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices:  Vec<u16>,
}

impl Mesh {
    pub fn clear(&mut self){
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn reserve_vertex(&mut self,additional:usize){
        self.vertices.reserve(additional);
    }

    pub fn reserve_index(&mut self,additional:usize){
        self.indices.reserve(additional * 3);
    }

    pub fn add_triangle(&mut self , point_1:u16 , point_2:u16 , point_3:u16){
      
        self.indices.push(point_1);
        self.indices.push(point_2);
        self.indices.push(point_3);
    }

    pub fn color_triangle(&mut self , pos:Vector2f , color:Color){

        self.vertices.push(Vertex { pos, uv: Vector2f::ZERO, color })
    }

    pub fn text_triangle(&mut self , pos:Vector2f , uv:Vector2f){

        self.vertices.push(Vertex { pos, uv, color:Color::WHITE })
    }
}












