extern crate euclid;

mod shapes;
mod utils;
mod macros;

use shapes::rectangle::Rectangle;
use shapes::triangle::Triangle;
use shapes::circle::Circle;

pub type Point = euclid::Point2D<f32, f32>;
pub type Points = Vec<Point>;
pub type Size = euclid::Size2D<f32, f32>;

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub width: f32,
    pub height: f32
}

pub fn rectangle(name: String, x: f32, y: f32, width: f32, height: f32) -> Rectangle {
    return Rectangle::new(name,Point::new(x, y), Size::new(width, height));
}

pub fn triangle(name: String, x: f32, y: f32, width: f32, height: f32) -> Triangle {
    return Triangle::new(name,Point::new(x, y), Size::new(width, height));
}

pub fn circle(name: String, x: f32, y: f32, radius: f32, segments: i16) -> Circle {
    return Circle::new(name, Point::new(x, y), radius, segments);
}