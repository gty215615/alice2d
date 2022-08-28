

pub enum Shape {
    Null,
    Children(Vec<Shape>),
    Path(PathShape),
    Rect(RectShape),
    Circle(CircleShape)
}


pub struct PathShape {

}


pub struct RectShape {

}

pub struct CircleShape {

}
