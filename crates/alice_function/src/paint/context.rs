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

pub fn stroke_path(feathering:f32 ,points:&[PathPoint] , stroke:Stroke , paint_type:PathType, out:&mut Mesh ) {

  
    let idx = out.vertices.len() as u16;
    let n = points.len();



    // 是否启用反走样

    if feathering > 0.0 {

        let outer_color = Color::TRANSPARENT;
        let inner_color = stroke.color;
        // 判断是粗线还是细线
        let is_thin_line = stroke.width < feathering;

        if is_thin_line {
            // 细线
            let inner_color = inner_color * ( stroke.width / feathering );

            if inner_color == Color::TRANSPARENT {
       
                return ;
            }
          
            out.reserve_vertex( n * 3);
            out.reserve_index(n * 4);

            let step = 3_u16;


            let mut i0 = n - 1;
            for i1 in 0..n {
                let is_connect_prev_point = if paint_type == PathType::Closed || i1 > 0 { true } else { false };
                let pos = points[i1].pos;
                let normal = points[i1].normal;

                    // [ 0 , 1 , 3 ]
                    // [ 1 , 3 , 4 ]
                    // [ 1,  2,  4 ]
                    // [ 2 , 4 , 5 ] 


                    // [ 3 , 4 , 6 ] 
                    // [ 4 , 6 , 7 ] 
                    // [ 4 , 5 , 7 ] 
                    // [ 5 , 7 , 8 ] 

          
                out.color_triangle(pos + normal * stroke.width, outer_color);  // outer
                out.color_triangle(pos, inner_color);                              // inner
                out.color_triangle(pos - normal * stroke.width, outer_color);  // outer

                if is_connect_prev_point {
                    let i0 = i0  as u16;
                    let i1 = i1  as u16;
                    out.add_triangle(idx + i0 * step + 0, idx + i0 * step + 1, idx + i1 * step + 0); 
                    out.add_triangle(idx + i0 * step + 1, idx + i1 * step + 0, idx + i1 * step + 1); 
    
    
                    out.add_triangle(idx + i0 * step + 1, idx + i0 * step + 2, idx + i1 * step + 1); 
                    out.add_triangle(idx + i0 * step + 2, idx + i1 * step + 1, idx + i1 * step + 2); 
    
    
                }
                i0 = i1;

            }

        }else{
            // 粗线

            out.reserve_vertex( n * 4);
            out.reserve_index(n * 6);

            let step = 4_u16;
            match paint_type {
                PathType::Closed => {
                    // 对于闭合路径
                    
                    let mut i0 = n - 1;

                    let feathering_width = (stroke.width - feathering) / 2.0;

                    for i1 in 0..n {
                        let pos = points[i1].pos;
                        let normal = points[i1].normal;

                        

                        out.color_triangle(pos + normal * (stroke.width + feathering_width), outer_color);  // inner
                        out.color_triangle(pos + normal * stroke.width, inner_color);  // inner
   
                        out.color_triangle(pos - normal * stroke.width, inner_color);  // outer
                        out.color_triangle(pos - normal * (stroke.width + feathering_width), outer_color);  // outer
        
                        // [ 0 , 1 , 4 ]
                        // [ 1 , 4 , 5 ]
                        // [ 1 , 5 , 6 ]
                        // [ 1 , 2 , 6 ]
                        // [ 2 , 6 , 7 ]
                        // [ 2 , 3 , 7 ]
                        {
                            let i0 = i0  as u16;
                            let i1 = i1  as u16;

                            out.add_triangle(idx + i0 * step + 0, idx + i0 * step + 1, idx + i1 * step + 0); 
                            out.add_triangle(idx + i0 * step + 1, idx + i1 * step + 0, idx + i1 * step + 1); 
            
            
                            out.add_triangle(idx + i0 * step + 1, idx + i1 * step + 1, idx + i1 * step + 2); 
                            out.add_triangle(idx + i0 * step + 1, idx + i0 * step + 2, idx + i1 * step + 2); 
            

                            out.add_triangle(idx + i0 * step + 2, idx + i1 * step + 2, idx + i1 * step + 3); 
                            out.add_triangle(idx + i0 * step + 2, idx + i0 * step + 3, idx + i1 * step + 3); 
            
                        }
                        
                        i0 = i1;


                    }

                }
                PathType::Open => {
                    // TODO 对于不闭合的路径两端点要进行单独处理 因为他们会有斜切头 
                }
            }


        }

    }else{



            out.reserve_vertex( n * 2);
            out.reserve_index(n * 2);





            let step = 2_u16;
            let mut i0 = n - 1;
            for i1 in 0..n {
                
                let is_connect_prev_point = if paint_type == PathType::Closed || i1 > 0 { true } else { false };

                out.color_triangle(points[i1].pos + points[i1].normal * stroke.width, stroke.color);
                out.color_triangle(points[i1].pos - points[i1].normal * stroke.width, stroke.color);

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



}