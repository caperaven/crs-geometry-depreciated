use crate::{Point, Points, Size, BoundingBox};
use crate::utils::standard_aabb::get_bounding;

pub struct Rectangle {
    pub points: Points,
    pub indices: Vec<i8>,
    pub origin: Point,
    pub aabb: BoundingBox
}

impl Rectangle {
    pub fn new (origin: Point, size: Size) -> Rectangle {
        let parts = get_bounding(origin, size);

        let mut points: Points = Vec::new();
        points.push(Point::new(parts.min_x, parts.min_y));
        points.push(Point::new(parts.max_x, parts.min_y));
        points.push(Point::new(parts.max_x, parts.max_y));
        points.push(Point::new(parts.min_x, parts.max_y));

        let indices = vec![0, 1, 2, 0, 2, 3];

        Rectangle {
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
    fn create_rectangle() {
        let rectangle = Rectangle::new(Point::new(0, 0), Size::new(200, 200));

        assert_eq!(rectangle.origin.x, 0);
        assert_eq!(rectangle.origin.y, 0);
        assert_eq!(rectangle.points.len(), 4);

        assert_eq!(rectangle.points[0].x, -100);
        assert_eq!(rectangle.points[0].y, -100);
        assert_eq!(rectangle.points[1].x, 100);
        assert_eq!(rectangle.points[1].y, -100);
        assert_eq!(rectangle.points[2].x, 100);
        assert_eq!(rectangle.points[2].y, 100);
        assert_eq!(rectangle.points[3].x, -100);
        assert_eq!(rectangle.points[3].y, 100);

        assert_eq!(rectangle.indices.len(), 6);
        assert_eq!(rectangle.indices[0], 0);
        assert_eq!(rectangle.indices[1], 1);
        assert_eq!(rectangle.indices[2], 2);
        assert_eq!(rectangle.indices[3], 0);
        assert_eq!(rectangle.indices[4], 2);
        assert_eq!(rectangle.indices[5], 3);

        assert_eq!(rectangle.aabb.min_x, -100);
        assert_eq!(rectangle.aabb.max_x, 100);
        assert_eq!(rectangle.aabb.min_y, -100);
        assert_eq!(rectangle.aabb.max_y, 100);
    }
}