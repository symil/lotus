use crate::Point;

pub enum BodyShape {
    Circle(f64),
    Polygon(Vec<Point>)
}