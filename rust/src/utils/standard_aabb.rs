use crate::{Point, Size, BoundingBox};

pub struct BoundingParts {
    pub half_width: i32,
    pub half_height: i32,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub aabb: BoundingBox
}

pub fn get_bounding(origin: Point, size: Size) -> BoundingParts {
    let half_width = size.width / 2;
    let half_height = size.height / 2;
    let min_x = origin.x - half_width;
    let max_x = origin.x + half_width;
    let min_y = origin.y - half_height;
    let max_y = origin.y + half_height;
    let aabb = BoundingBox {min_x, max_x, min_y, max_y};

    return BoundingParts {
        half_width,
        half_height,
        min_x,
        max_x,
        min_y,
        max_y,
        aabb
    };
}