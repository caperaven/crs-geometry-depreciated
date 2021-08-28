use crate::{Point, Points, Size, BoundingBox};
use crate::utils::standard_aabb::get_bounding;
use crate::str_to_i32;

pub struct Triangle {
    pub name: String,
    pub points: Points,
    pub indices: Vec<i8>,
    pub origin: Point,
    pub aabb: BoundingBox
}

impl Triangle {
    pub fn new(name: String, origin: Point, size: Size) -> Triangle {
        let parts = get_bounding(origin, size);

        let mut points: Points = Vec::new();
        points.push(Point::new(origin.x, parts.min_y));
        points.push(Point::new(parts.max_x, parts.max_y));
        points.push(Point::new(parts.min_x, parts.max_y));

        let indices = vec![0, 1, 2];

        Triangle {
            name,
            points,
            indices,
            origin,
            aabb: parts.aabb
        }
    }
}

/// Convert a String to Triangle
/// Structure: "name,origin_x,origin_y,width,height"
/// Example: "triangle 1,0,0,200,200"
impl From<String> for Triangle {
    fn from(def: String) -> Triangle {
        let parts: Vec<&str> = def.split(",").collect();

        let name = String::from(parts[0]);
        let x: i32 = str_to_i32!(parts[1]);
        let y: i32 = str_to_i32!(parts[2]);
        let width: i32 = str_to_i32!(parts[3]);
        let height: i32 = str_to_i32!(parts[4]);

        return Triangle::new(name,Point::new(x, y), Size::new(width, height));
    }
}

impl From<Triangle> for String {
    fn from(triangle: Triangle) -> String {
        let mut parts: Vec<String> = Vec::new();
        parts.push(triangle.name);
        parts.push(triangle.origin.x.to_string());
        parts.push(triangle.origin.y.to_string());
        parts.push(triangle.aabb.width.to_string());
        parts.push(triangle.aabb.height.to_string());
        return parts.join(",");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_triangle() {
        let triangle = Triangle::new("Triangle 1".into(),Point::new(0, 0), Size::new(200, 200));
        assert_triangle(&triangle);
    }

    #[test]
    fn triangle_from() {
        let value = String::from("Triangle 1,0,0,200,200");
        let triangle: Triangle = Triangle::from(value);
        assert_triangle(&triangle);
    }

    #[test]
    fn triangle_to_string() {
        let triangle = Triangle::new("Triangle 1".into(),Point::new(0, 0), Size::new(200, 200));
        let result: String = triangle.into();
        assert_eq!(result, "Triangle 1,0,0,200,200")
    }

    fn assert_triangle(triangle: &Triangle) {
        assert_eq!(triangle.name, "Triangle 1");

        assert_eq!(triangle.origin.x, 0);
        assert_eq!(triangle.origin.y, 0);
        assert_eq!(triangle.points.len(), 3);

        assert_eq!(triangle.points[0].x, 0);
        assert_eq!(triangle.points[0].y, -100);
        assert_eq!(triangle.points[1].x, 100);
        assert_eq!(triangle.points[1].y, 100);
        assert_eq!(triangle.points[2].x, -100);
        assert_eq!(triangle.points[2].y, 100);

        assert_eq!(triangle.indices.len(), 3);
        assert_eq!(triangle.indices[0], 0);
        assert_eq!(triangle.indices[1], 1);
        assert_eq!(triangle.indices[2], 2);

        assert_eq!(triangle.aabb.min_x, -100);
        assert_eq!(triangle.aabb.max_x, 100);
        assert_eq!(triangle.aabb.min_y, -100);
        assert_eq!(triangle.aabb.max_y, 100);
    }
}