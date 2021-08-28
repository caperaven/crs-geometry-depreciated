extern crate euclid;

mod shapes;
mod utils;
mod macros;

use shapes::rectangle::Rectangle;
use shapes::triangle::Triangle;

pub type Point = euclid::Point2D<i32, i32>;
pub type Points = Vec<Point>;
pub type Size = euclid::Size2D<i32, i32>;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub width: i32,
    pub height: i32
}

pub fn rectangle(name: String, x: i32, y: i32, width: i32, height: i32) -> Rectangle {
    return Rectangle::new(name,Point::new(x, y), Size::new(width, height));
}

pub fn triangle(name: String, x: i32, y: i32, width: i32, height: i32) -> Triangle {
    return Triangle::new(name,Point::new(x, y), Size::new(width, height));
}