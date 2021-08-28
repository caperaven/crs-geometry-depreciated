extern crate euclid;

mod shapes;
mod utils;
mod macros;

use shapes::rectangle::Rectangle;
use shapes::triangle::Triangle;

type Point = euclid::Point2D<i32, i32>;
type Points = Vec<Point>;
type Size = euclid::Size2D<i32, i32>;

pub struct BoundingBox {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32
}

pub fn rectangle(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
    return Rectangle::new(Point::new(x, y), Size::new(width, height));
}

pub fn triangle(x: i32, y: i32, width: i32, height: i32) -> Triangle {
    return Triangle::new(Point::new(x, y), Size::new(width, height));
}