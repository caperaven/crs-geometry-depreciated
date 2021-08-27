use crate::{Point, Points, Size, BoundingBox};
use crate::utils::standard_aabb::get_bounding;

pub struct Triangle {
    pub points: Points,
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

        Triangle {
            points,
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
        let rectangle = Triangle::new(Point::new(0, 0), Size::new(200, 200));

        assert_eq!(rectangle.origin.x, 0);
        assert_eq!(rectangle.origin.y, 0);
        assert_eq!(rectangle.points.len(), 3);

        assert_eq!(rectangle.points[0].x, 0);
        assert_eq!(rectangle.points[0].y, -100);
        assert_eq!(rectangle.points[1].x, 100);
        assert_eq!(rectangle.points[1].y, 100);
        assert_eq!(rectangle.points[2].x, -100);
        assert_eq!(rectangle.points[2].y, 100);
    }
}