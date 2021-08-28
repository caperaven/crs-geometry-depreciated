use crate::{Point, Points, Size, BoundingBox};
use crate::utils::standard_aabb::get_bounding;

pub struct Triangle {
    pub points: Points,
    pub indices: Vec<i8>,
    pub origin: Point,
    pub aabb: BoundingBox
}

impl Triangle {
    pub fn new(origin: Point, size: Size) -> Triangle {
        let parts = get_bounding(origin, size);

        let mut points: Points = Vec::new();
        points.push(Point::new(origin.x, parts.min_y));
        points.push(Point::new(parts.max_x, parts.max_y));
        points.push(Point::new(parts.min_x, parts.max_y));

        let indices = vec![0, 1, 2];

        Triangle {
            points,
            indices,
            origin,
            aabb: parts.aabb
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_triangle() {
        let triangle = Triangle::new(Point::new(0, 0), Size::new(200, 200));

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