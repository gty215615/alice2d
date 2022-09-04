use alice_core::{math::Vector2f, color::Color};

use crate::geometry::rect::Rect;

use super::shape::Round;



pub const CIRCLE_8: [Vector2f; 9] = [
    Vector2f {x: 1.000000, y: 0.000000 },
    Vector2f {x: 0.707107, y: 0.707107 },
    Vector2f {x: 0.000000, y: 1.000000 },
    Vector2f {x: -0.707107, y: 0.707107 },
    Vector2f {x: -1.000000, y: 0.000000 },
    Vector2f {x: -0.707107, y: -0.707107 },
    Vector2f {x: 0.000000, y: -1.000000 },
    Vector2f {x: 0.707107, y: -0.707107 },
    Vector2f {x: 1.000000, y: 0.000000 },
];

pub const CIRCLE_16: [Vector2f; 17] = [
    Vector2f {x: 1.000000, y: 0.000000 },
    Vector2f {x: 0.923880, y: 0.382683 },
    Vector2f {x: 0.707107, y: 0.707107 },
    Vector2f {x: 0.382683, y: 0.923880 },
    Vector2f {x: 0.000000, y: 1.000000 },
    Vector2f {x: -0.382684, y: 0.923880 },
    Vector2f {x: -0.707107, y: 0.707107 },
    Vector2f {x: -0.923880, y: 0.382683 },
    Vector2f {x: -1.000000, y: 0.000000 },
    Vector2f {x: -0.923880, y: -0.382683 },
    Vector2f {x: -0.707107, y: -0.707107 },
    Vector2f {x: -0.382684, y: -0.923880 },
    Vector2f {x: 0.000000, y: -1.000000 },
    Vector2f {x: 0.382684, y: -0.923879 },
    Vector2f {x: 0.707107, y: -0.707107 },
    Vector2f {x: 0.923880, y: -0.382683 },
    Vector2f {x: 1.000000, y: 0.000000 },
];

pub const CIRCLE_32: [Vector2f; 33] = [
    Vector2f {x: 1.000000, y: 0.000000 },
    Vector2f {x: 0.980785, y: 0.195090 },
    Vector2f {x: 0.923880, y: 0.382683 },
    Vector2f {x: 0.831470, y: 0.555570 },
    Vector2f {x: 0.707107, y: 0.707107 },
    Vector2f {x: 0.555570, y: 0.831470 },
    Vector2f {x: 0.382683, y: 0.923880 },
    Vector2f {x: 0.195090, y: 0.980785 },
    Vector2f {x: 0.000000, y: 1.000000 },
    Vector2f {x: -0.195090, y: 0.980785 },
    Vector2f {x: -0.382683, y: 0.923880 },
    Vector2f {x: -0.555570, y: 0.831470 },
    Vector2f {x: -0.707107, y: 0.707107 },
    Vector2f {x: -0.831470, y: 0.555570 },
    Vector2f {x: -0.923880, y: 0.382683 },
    Vector2f {x: -0.980785, y: 0.195090 },
    Vector2f {x: -1.000000, y: 0.000000 },
    Vector2f {x: -0.980785, y: -0.195090 },
    Vector2f {x: -0.923880, y: -0.382683 },
    Vector2f {x: -0.831470, y: -0.555570 },
    Vector2f {x: -0.707107, y: -0.707107 },
    Vector2f {x: -0.555570, y: -0.831470 },
    Vector2f {x: -0.382683, y: -0.923880 },
    Vector2f {x: -0.195090, y: -0.980785 },
    Vector2f {x: -0.000000, y: -1.000000 },
    Vector2f {x: 0.195090, y: -0.980785 },
    Vector2f {x: 0.382683, y: -0.923880 },
    Vector2f {x: 0.555570, y: -0.831470 },
    Vector2f {x: 0.707107, y: -0.707107 },
    Vector2f {x: 0.831470, y: -0.555570 },
    Vector2f {x: 0.923880, y: -0.382683 },
    Vector2f {x: 0.980785, y: -0.195090 },
    Vector2f {x: 1.000000, y: -0.000000 },
];

pub const CIRCLE_64: [Vector2f; 65] = [
    Vector2f {x: 1.000000, y: 0.000000 },
    Vector2f {x: 0.995185, y: 0.098017 },
    Vector2f {x: 0.980785, y: 0.195090 },
    Vector2f {x: 0.956940, y: 0.290285 },
    Vector2f {x: 0.923880, y: 0.382683 },
    Vector2f {x: 0.881921, y: 0.471397 },
    Vector2f {x: 0.831470, y: 0.555570 },
    Vector2f {x: 0.773010, y: 0.634393 },
    Vector2f {x: 0.707107, y: 0.707107 },
    Vector2f {x: 0.634393, y: 0.773010 },
    Vector2f {x: 0.555570, y: 0.831470 },
    Vector2f {x: 0.471397, y: 0.881921 },
    Vector2f {x: 0.382683, y: 0.923880 },
    Vector2f {x: 0.290285, y: 0.956940 },
    Vector2f {x: 0.195090, y: 0.980785 },
    Vector2f {x: 0.098017, y: 0.995185 },
    Vector2f {x: 0.000000, y: 1.000000 },
    Vector2f {x: -0.098017, y: 0.995185 },
    Vector2f {x: -0.195090, y: 0.980785 },
    Vector2f {x: -0.290285, y: 0.956940 },
    Vector2f {x: -0.382683, y: 0.923880 },
    Vector2f {x: -0.471397, y: 0.881921 },
    Vector2f {x: -0.555570, y: 0.831470 },
    Vector2f {x: -0.634393, y: 0.773010 },
    Vector2f {x: -0.707107, y: 0.707107 },
    Vector2f {x: -0.773010, y: 0.634393 },
    Vector2f {x: -0.831470, y: 0.555570 },
    Vector2f {x: -0.881921, y: 0.471397 },
    Vector2f {x: -0.923880, y: 0.382683 },
    Vector2f {x: -0.956940, y: 0.290285 },
    Vector2f {x: -0.980785, y: 0.195090 },
    Vector2f {x: -0.995185, y: 0.098017 },
    Vector2f {x: -1.000000, y: 0.000000 },
    Vector2f {x: -0.995185, y: -0.098017 },
    Vector2f {x: -0.980785, y: -0.195090 },
    Vector2f {x: -0.956940, y: -0.290285 },
    Vector2f {x: -0.923880, y: -0.382683 },
    Vector2f {x: -0.881921, y: -0.471397 },
    Vector2f {x: -0.831470, y: -0.555570 },
    Vector2f {x: -0.773010, y: -0.634393 },
    Vector2f {x: -0.707107, y: -0.707107 },
    Vector2f {x: -0.634393, y: -0.773010 },
    Vector2f {x: -0.555570, y: -0.831470 },
    Vector2f {x: -0.471397, y: -0.881921 },
    Vector2f {x: -0.382683, y: -0.923880 },
    Vector2f {x: -0.290285, y: -0.956940 },
    Vector2f {x: -0.195090, y: -0.980785 },
    Vector2f {x: -0.098017, y: -0.995185 },
    Vector2f {x: -0.000000, y: -1.000000 },
    Vector2f {x: 0.098017, y: -0.995185 },
    Vector2f {x: 0.195090, y: -0.980785 },
    Vector2f {x: 0.290285, y: -0.956940 },
    Vector2f {x: 0.382683, y: -0.923880 },
    Vector2f {x: 0.471397, y: -0.881921 },
    Vector2f {x: 0.555570, y: -0.831470 },
    Vector2f {x: 0.634393, y: -0.773010 },
    Vector2f {x: 0.707107, y: -0.707107 },
    Vector2f {x: 0.773010, y: -0.634393 },
    Vector2f {x: 0.831470, y: -0.555570 },
    Vector2f {x: 0.881921, y: -0.471397 },
    Vector2f {x: 0.923880, y: -0.382683 },
    Vector2f {x: 0.956940, y: -0.290285 },
    Vector2f {x: 0.980785, y: -0.195090 },
    Vector2f {x: 0.995185, y: -0.098017 },
    Vector2f {x: 1.000000, y: -0.000000 },
];

pub const CIRCLE_128: [Vector2f; 129] = [
    Vector2f {x: 1.000000, y: 0.000000 },
    Vector2f {x: 0.998795, y: 0.049068 },
    Vector2f {x: 0.995185, y: 0.098017 },
    Vector2f {x: 0.989177, y: 0.146730 },
    Vector2f {x: 0.980785, y: 0.195090 },
    Vector2f {x: 0.970031, y: 0.242980 },
    Vector2f {x: 0.956940, y: 0.290285 },
    Vector2f {x: 0.941544, y: 0.336890 },
    Vector2f {x: 0.923880, y: 0.382683 },
    Vector2f {x: 0.903989, y: 0.427555 },
    Vector2f {x: 0.881921, y: 0.471397 },
    Vector2f {x: 0.857729, y: 0.514103 },
    Vector2f {x: 0.831470, y: 0.555570 },
    Vector2f {x: 0.803208, y: 0.595699 },
    Vector2f {x: 0.773010, y: 0.634393 },
    Vector2f {x: 0.740951, y: 0.671559 },
    Vector2f {x: 0.707107, y: 0.707107 },
    Vector2f {x: 0.671559, y: 0.740951 },
    Vector2f {x: 0.634393, y: 0.773010 },
    Vector2f {x: 0.595699, y: 0.803208 },
    Vector2f {x: 0.555570, y: 0.831470 },
    Vector2f {x: 0.514103, y: 0.857729 },
    Vector2f {x: 0.471397, y: 0.881921 },
    Vector2f {x: 0.427555, y: 0.903989 },
    Vector2f {x: 0.382683, y: 0.923880 },
    Vector2f {x: 0.336890, y: 0.941544 },
    Vector2f {x: 0.290285, y: 0.956940 },
    Vector2f {x: 0.242980, y: 0.970031 },
    Vector2f {x: 0.195090, y: 0.980785 },
    Vector2f {x: 0.146730, y: 0.989177 },
    Vector2f {x: 0.098017, y: 0.995185 },
    Vector2f {x: 0.049068, y: 0.998795 },
    Vector2f {x: 0.000000, y: 1.000000 },
    Vector2f {x: -0.049068, y: 0.998795 },
    Vector2f {x: -0.098017, y: 0.995185 },
    Vector2f {x: -0.146730, y: 0.989177 },
    Vector2f {x: -0.195090, y: 0.980785 },
    Vector2f {x: -0.242980, y: 0.970031 },
    Vector2f {x: -0.290285, y: 0.956940 },
    Vector2f {x: -0.336890, y: 0.941544 },
    Vector2f {x: -0.382683, y: 0.923880 },
    Vector2f {x: -0.427555, y: 0.903989 },
    Vector2f {x: -0.471397, y: 0.881921 },
    Vector2f {x: -0.514103, y: 0.857729 },
    Vector2f {x: -0.555570, y: 0.831470 },
    Vector2f {x: -0.595699, y: 0.803208 },
    Vector2f {x: -0.634393, y: 0.773010 },
    Vector2f {x: -0.671559, y: 0.740951 },
    Vector2f {x: -0.707107, y: 0.707107 },
    Vector2f {x: -0.740951, y: 0.671559 },
    Vector2f {x: -0.773010, y: 0.634393 },
    Vector2f {x: -0.803208, y: 0.595699 },
    Vector2f {x: -0.831470, y: 0.555570 },
    Vector2f {x: -0.857729, y: 0.514103 },
    Vector2f {x: -0.881921, y: 0.471397 },
    Vector2f {x: -0.903989, y: 0.427555 },
    Vector2f {x: -0.923880, y: 0.382683 },
    Vector2f {x: -0.941544, y: 0.336890 },
    Vector2f {x: -0.956940, y: 0.290285 },
    Vector2f {x: -0.970031, y: 0.242980 },
    Vector2f {x: -0.980785, y: 0.195090 },
    Vector2f {x: -0.989177, y: 0.146730 },
    Vector2f {x: -0.995185, y: 0.098017 },
    Vector2f {x: -0.998795, y: 0.049068 },
    Vector2f {x: -1.000000, y: 0.000000 },
    Vector2f {x: -0.998795, y: -0.049068 },
    Vector2f {x: -0.995185, y: -0.098017 },
    Vector2f {x: -0.989177, y: -0.146730 },
    Vector2f {x: -0.980785, y: -0.195090 },
    Vector2f {x: -0.970031, y: -0.242980 },
    Vector2f {x: -0.956940, y: -0.290285 },
    Vector2f {x: -0.941544, y: -0.336890 },
    Vector2f {x: -0.923880, y: -0.382683 },
    Vector2f {x: -0.903989, y: -0.427555 },
    Vector2f {x: -0.881921, y: -0.471397 },
    Vector2f {x: -0.857729, y: -0.514103 },
    Vector2f {x: -0.831470, y: -0.555570 },
    Vector2f {x: -0.803208, y: -0.595699 },
    Vector2f {x: -0.773010, y: -0.634393 },
    Vector2f {x: -0.740951, y: -0.671559 },
    Vector2f {x: -0.707107, y: -0.707107 },
    Vector2f {x: -0.671559, y: -0.740951 },
    Vector2f {x: -0.634393, y: -0.773010 },
    Vector2f {x: -0.595699, y: -0.803208 },
    Vector2f {x: -0.555570, y: -0.831470 },
    Vector2f {x: -0.514103, y: -0.857729 },
    Vector2f {x: -0.471397, y: -0.881921 },
    Vector2f {x: -0.427555, y: -0.903989 },
    Vector2f {x: -0.382683, y: -0.923880 },
    Vector2f {x: -0.336890, y: -0.941544 },
    Vector2f {x: -0.290285, y: -0.956940 },
    Vector2f {x: -0.242980, y: -0.970031 },
    Vector2f {x: -0.195090, y: -0.980785 },
    Vector2f {x: -0.146730, y: -0.989177 },
    Vector2f {x: -0.098017, y: -0.995185 },
    Vector2f {x: -0.049068, y: -0.998795 },
    Vector2f {x: -0.000000, y: -1.000000 },
    Vector2f {x: 0.049068, y: -0.998795 },
    Vector2f {x: 0.098017, y: -0.995185 },
    Vector2f {x: 0.146730, y: -0.989177 },
    Vector2f {x: 0.195090, y: -0.980785 },
    Vector2f {x: 0.242980, y: -0.970031 },
    Vector2f {x: 0.290285, y: -0.956940 },
    Vector2f {x: 0.336890, y: -0.941544 },
    Vector2f {x: 0.382683, y: -0.923880 },
    Vector2f {x: 0.427555, y: -0.903989 },
    Vector2f {x: 0.471397, y: -0.881921 },
    Vector2f {x: 0.514103, y: -0.857729 },
    Vector2f {x: 0.555570, y: -0.831470 },
    Vector2f {x: 0.595699, y: -0.803208 },
    Vector2f {x: 0.634393, y: -0.773010 },
    Vector2f {x: 0.671559, y: -0.740951 },
    Vector2f {x: 0.707107, y: -0.707107 },
    Vector2f {x: 0.740951, y: -0.671559 },
    Vector2f {x: 0.773010, y: -0.634393 },
    Vector2f {x: 0.803208, y: -0.595699 },
    Vector2f {x: 0.831470, y: -0.555570 },
    Vector2f {x: 0.857729, y: -0.514103 },
    Vector2f {x: 0.881921, y: -0.471397 },
    Vector2f {x: 0.903989, y: -0.427555 },
    Vector2f {x: 0.923880, y: -0.382683 },
    Vector2f {x: 0.941544, y: -0.336890 },
    Vector2f {x: 0.956940, y: -0.290285 },
    Vector2f {x: 0.970031, y: -0.242980 },
    Vector2f {x: 0.980785, y: -0.195090 },
    Vector2f {x: 0.989177, y: -0.146730 },
    Vector2f {x: 0.995185, y: -0.098017 },
    Vector2f {x: 0.998795, y: -0.049068 },
    Vector2f {x: 1.000000, y: -0.000000 },
];



#[derive(Debug,PartialEq, PartialOrd)]
pub enum PathType {
    Open,
    Closed
}


/// 
/// 路径由无数个点 (PathPoint) 组成
/// 点是路径组成的最小单元
/// 点具有位置 (pos) 以及 延展方向 (normal) 也是路径的宽度 
/// 通常 两个点连接成一条线段 (segment) ,线段具有宽度 , 沿着两点组成的向量的法向正方向与法向负方向同时延伸形成线段的宽
/// 

pub struct PathPoint {
    pub pos:    Vector2f,
    pub normal: Vector2f
}


pub struct Path(pub(crate) Vec<PathPoint>);


impl Path {
    pub fn new() -> Self {
        Path(Vec::new())
    }
    pub fn reserve(&mut self , additional:usize){
        self.0.reserve(additional)
    }

    pub fn clear(&mut self){
        self.0.clear()
    }

    pub fn add_point(&mut self , pos:Vector2f , normal:Vector2f){
        self.0.push(
            PathPoint { pos, normal}
        );
    }

    pub fn add_circle(&mut self , center:Vector2f , radius:f32){
        if radius <= 2.0 {
            self.0.extend(CIRCLE_8.iter().map(|&n| PathPoint {
                pos: center + n * radius,
                normal: n,
            }));
        } else if radius <= 5.0 {
            self.0.extend(CIRCLE_16.iter().map(|&n| PathPoint {
                pos: center + n * radius,
                normal: n,
            }));
        } else if radius < 18.0 {
            self.0.extend(CIRCLE_32.iter().map(|&n| PathPoint {
                pos: center + n * radius,
                normal: n,
            }));
        } else if radius < 50.0 {
            self.0.extend(CIRCLE_64.iter().map(|&n| PathPoint {
                pos: center + n * radius,
                normal: n,
            }));
        } else {
            self.0.extend(CIRCLE_128.iter().map(|&n| PathPoint {
                pos: center + n * radius,
                normal: n,
            }));
        }
    }

    pub fn add_line_segment(&mut self , pos:[Vector2f;2]) {
        self.reserve(2);
        // 获取到线段法向量
        let normal = (pos[1] - pos[0]).normalize().rot_90();
        self.add_point(pos[0], normal);
        self.add_point(pos[1], normal);
    }

    pub fn add_line_opened(&mut self , positions:Vec<Vector2f>) {
        let n = positions.len();
        assert!( n > 1);
        if n == 2 {
            self.add_line_segment([positions[0],positions[1]])
            
        }

        self.reserve(n);

        

        // 由于目前不考虑路径闭合的情形 因此 第一个点与最后一个点要单独算
        let mut n0 = (positions[1] - positions[0]).normalize().rot_90();
        self.add_point(positions[0], n0);
        for i in 1..n - 1 {
            let mut n1 = (positions[i + 1] - positions[i]).normalize().rot_90();
            if n0 == Vector2f::ZERO {
                n0 = n1;
            }else if n1 == Vector2f::ZERO {
                n1 = n0;
            }
            let normal = ((n0 + n1) / 2.0).normalize();

            // TODO! 夹角很小情况
            self.add_point(positions[i], normal);
            n0 = n1;
        }

        // 计算最后一个点
        self.add_point(positions[n - 1], (positions[n - 1] - positions[ n - 2]).normalize().rot_90());



    }

    pub fn add_line_closed(&mut self , positions:Vec<Vector2f>) {
        let n = positions.len();

        assert!(n > 2);

        self.reserve(n);

    
        let mut n0 = (positions[0] - positions[n - 1]).normalize().rot_90();
   
        for i in 0..n {
            let next_id = if i + 1 == n { 0 } else { i + 1 };
            let mut n1 = (positions[next_id] - positions[i]).normalize().rot_90();
            if n0 == Vector2f::ZERO {
                n0 = n1;
            }else if n1 == Vector2f::ZERO {
                n1 = n0;
            }
            let normal = ((n0 + n1) / 2.0).normalize();
            // TODO! 夹角很小情况
            self.add_point(positions[i], normal);
            n0 = n1;
        }

    }


    pub fn add_rounded_rectangle( path:&mut Vec<Vector2f> , rect:Rect , r:Round ) {

        if r.is_empty() {
            path.push( rect.left_top() );
            path.push( rect.right_top() );
            path.push( rect.right_bottom() );
            path.push( rect.left_bottom() );
        }else {
            add_circle_quadrant(path, Vector2f::new(rect.max_point.x - r.se, rect.max_point.y - r.se), r.se, 0);
            add_circle_quadrant(path, Vector2f::new(rect.min_point.x + r.sw, rect.max_point.y - r.sw), r.sw, 1);
            add_circle_quadrant(path, Vector2f::new(rect.min_point.x + r.nw, rect.min_point.y + r.nw), r.nw, 2);
            add_circle_quadrant(path, Vector2f::new(rect.max_point.x - r.ne, rect.min_point.y + r.ne), r.ne, 3);
        }

    }


}












pub fn add_circle_quadrant( path:&mut Vec<Vector2f> , center:Vector2f , radius:f32 , quadrant: usize ){
    if radius < 0.0 {
        path.push(center);
    }else if radius < 2.0 {
        let offset = quadrant * 2;
        let quadrant_vertices = &CIRCLE_8[offset..=offset+2];
        path.extend(quadrant_vertices.iter().map(|&n| center + n * radius));
    }else if radius <= 5.0 {
        let offset = quadrant * 4;
        let quadrant_vertices = &CIRCLE_16[offset..=offset + 4];
        path.extend(quadrant_vertices.iter().map(|&n| center + n * radius));
    } else if radius < 18.0 {
        let offset = quadrant * 8;
        let quadrant_vertices = &CIRCLE_32[offset..=offset + 8];
        path.extend(quadrant_vertices.iter().map(|&n| center + n * radius));
    } else if radius < 50.0 {
        let offset = quadrant * 16;
        let quadrant_vertices = &CIRCLE_64[offset..=offset + 16];
        path.extend(quadrant_vertices.iter().map(|&n| center + n * radius));
    } else {
        let offset = quadrant * 32;
        let quadrant_vertices = &CIRCLE_128[offset..=offset + 32];
        path.extend(quadrant_vertices.iter().map(|&n| center + n * radius));
    }

}








