use crate::{Point, Points, Size, BoundingBox};
use crate::utils::standard_aabb::get_bounding;
//use crate::str_to_f32;

#[derive(Debug, Clone)]
pub struct Circle {
    pub name: String,
    pub points: Points,
    pub indices: Vec<i32>,
    pub origin: Point,
    pub aabb: BoundingBox,
    pub radius: f32
}

impl Circle {
    pub fn new(name: String, origin: Point, radius: f32, segments: i16) -> Circle {
        let rad_offset = std::f32::consts::PI/180.0;
        let size = radius * 2.0;
        let aabb_parts = get_bounding(&origin, &Size::new(size, size));
        let offset = 360 / segments;

        let mut points: Points = Vec::new();
        points.push(origin.clone());

        for segment in 0..segments {
            let value = rad_offset * (segment * offset) as f32;
            let mut x = value.sin() + origin.x;
            let mut y = value.cos() + origin.y;

            x = (x * 100.00).trunc() / 100.00;
            y = (y * 100.00).trunc() / 100.00;

            points.push(Point::new(x, y));
        }

        let mut indices = Vec::new();
        for i in 0..segments - 1 {
            let value = i as i32;
            indices.push(0);
            indices.push(value + 2);
            indices.push(value + 1);
        }
        indices.push(0);
        indices.push(1);
        indices.push(segments as i32);

        Circle {
            name,
            points,
            indices: indices,
            origin,
            aabb: aabb_parts.aabb,
            radius
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn circle_segments() {
        let circle = Circle::new("circle 1".into(), Point::new(0.0, 0.0), 1.0, 4);

        assert_eq!(circle.points.len(), 5);
        assert!(circle.points[0].x < 0.005);
        assert!(circle.points[0].y < 0.005);
        assert!(circle.points[1].x < 0.005);
        assert_eq!(circle.points[1].y, 1.0);
        assert_eq!(circle.points[2].x, 1.0);
        assert!(circle.points[2].y < 0.005);
        assert!(circle.points[3].x < 0.005);
        assert_eq!(circle.points[3].y, -1.0);
        assert_eq!(circle.points[4].x, -1.0);
        assert!(circle.points[4].y < 0.005);

        assert_eq!(circle.indices.len(), 12);
    }
}