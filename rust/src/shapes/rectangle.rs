use crate::{Point, Points, Size, BoundingBox};

pub struct Rectangle {
    pub points: Points,
    pub origin: Point,
    pub aabb: BoundingBox
}

impl Rectangle {
    pub fn new (origin: Point, size: Size) -> Rectangle {
        let half_width = size.width / 2;
        let half_height = size.height / 2;
        let min_x = origin.x - half_width;
        let max_x = origin.x + half_width;
        let min_y = origin.y - half_height;
        let max_y = origin.y + half_height;

        let aabb = BoundingBox {min_x, max_x, min_y, max_y};

        let mut points: Points = Vec::new();
        points.push(Point::new(min_x, min_y));
        points.push(Point::new(max_x, min_y));
        points.push(Point::new(max_x, max_y));
        points.push(Point::new(min_x, max_y));

        Rectangle {
            points,
            origin,
            aabb
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
    }
}