use alice_core::color::Color;

use super::{path::{PathPoint, Stroke, PathType}, mesh::Mesh};






pub struct Context {
     
}


impl Context {
    pub fn begin_frame(&mut self){

    }

    pub fn end_frame(&mut self){

    }

    
}


    ///                         stroke width
    ///                        |------|    
    ///neg normal   <-  |------p------|   -> normal
    ///                 |------|------|
    ///                 |------|------|
    ///                 |------|------|
    ///                 |------|------|
    ///                 |------|------|
    ///                 |------|------|
    ///                 |------|------|
    ///                 |------p------|
    /// 
pub fn stroke_line(points:&[PathPoint] , stroke:Stroke ,  out:&mut Mesh){
    
    let idx = out.vertices.len() as u16;

    out.reserve_vertex(4);
    out.reserve_index(6);

    out.color_triangle(points[0].pos + points[0].normal * stroke.width, Color::WHITE);
    out.color_triangle(points[0].pos - points[0].normal * stroke.width, Color::WHITE);

    out.color_triangle(points[1].pos + points[1].normal * stroke.width, Color::WHITE);
    out.color_triangle(points[1].pos - points[1].normal * stroke.width, Color::WHITE);

    out.add_triangle(idx + 0, idx + 2, idx + 3);
    out.add_triangle(idx + 0, idx + 1, idx + 3);
        

}

pub fn stroke_path(points:&[PathPoint] , stroke:Stroke , paint_type:PathType, out:&mut Mesh ) {

  
    let idx = out.vertices.len() as u16;
    let n = points.len();
    out.reserve_vertex( n * 2);
    out.reserve_index(n * 2);

    let step = 2_u16;
    let mut i0 = n - 1;
    for i1 in 0..n {
        
        let is_connect_prev_point = if paint_type == PathType::Closed || i1 > 0 { true } else { false };

        out.color_triangle(points[i1].pos + points[i1].normal * stroke.width, Color::WHITE);
        out.color_triangle(points[i1].pos - points[i1].normal * stroke.width, Color::WHITE);

        if is_connect_prev_point {
            let i0 = i0  as u16;
            let i1 = i1  as u16;

            // [ 4 , 5 , 1 ]
            // [ 4 , 0 , 1 ]
            // [ 0 , 1 , 3 ]
            // [ 0 , 2 , 3 ]
            // [ 2,  3,  5 ]
            // [ 2 , 4 , 5 ]                                                                                           // i0 
            out.add_triangle(idx + i0 * step + 0, idx + i0 * step + 1, idx + i1 * step + 1); // 4 5 1   0 1 3
            out.add_triangle(idx + i0 * step + 0, idx + i1 * step + 0, idx + i1 * step + 1); // 4 0 1   0 2 3
           
        }

        i0 = i1;
    }

}