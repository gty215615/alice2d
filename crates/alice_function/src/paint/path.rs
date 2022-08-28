use alice_core::{math::Vector2f, color::Color};


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

    pub fn add_point(&mut self , pos:Vector2f , normal:Vector2f){
        self.0.push(
            PathPoint { pos, normal}
        );
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
            self.add_point(positions[i], normal);
            n0 = n1;
        }

    }
}




#[derive(Debug , Clone, Copy)]
pub struct Stroke{
    pub width:f32,
    pub color:Color
}

impl std::default::Default for Stroke {
    fn default() -> Self {
        Self { width:1.0 , color: Color::WHITE }
    }
}





















