mod utils;

pub extern crate crs_geometry;

use wasm_bindgen::prelude::*;
use js_sys::Object;
use crate::utils::{
    set_property,
    set_point_array_property,
    set_int_array_property,
    set_point_property,
    set_aabb_property
};

#[wasm_bindgen]
pub fn get_rectangle(name: String, x: f32, y: f32, width: f32, height: f32) -> Object {
    let shape = crs_geometry::rectangle(name, x, y, width, height);
    let result = Object::new();

    set_property(&result, "name", &shape.name.into());
    set_point_array_property(&result, "points", &shape.points);
    set_int_array_property(&result, "indices", &shape.indices);
    set_point_property(&result, "origin", &shape.origin);
    set_aabb_property(&result, "aabb", &shape.aabb);

    return result;
}

#[wasm_bindgen]
pub fn get_triangle(name: String, x: f32, y: f32, width: f32, height: f32) -> Object {
    let shape = crs_geometry::triangle(name, x, y, width, height);
    let result = Object::new();

    set_property(&result, "name", &shape.name.into());
    set_point_array_property(&result, "points", &shape.points);
    set_int_array_property(&result, "indices", &shape.indices);
    set_point_property(&result, "origin", &shape.origin);
    set_aabb_property(&result, "aabb", &shape.aabb);

    return result;
}

#[wasm_bindgen]
pub fn get_circle(name: String, x: f32, y: f32, radius: f32, segments: i16) -> Object {
    let shape = crs_geometry::circle(name, x, y, radius, segments);
    let result = Object::new();

    set_property(&result, "name", &shape.name.into());
    set_point_array_property(&result, "points", &shape.points);
    set_int_array_property(&result, "indices", &shape.indices);
    set_point_property(&result, "origin", &shape.origin);
    set_aabb_property(&result, "aabb", &shape.aabb);

    return result;
}