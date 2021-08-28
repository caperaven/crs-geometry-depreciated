mod utils;

pub extern crate crs_geometry;

use wasm_bindgen::prelude::*;
use js_sys::Object;
use crate::utils::{set_property, set_point_array_property, set_int_array_property};

#[wasm_bindgen]
pub fn rectangle(name: String, x: i32, y: i32, width: i32, height: i32) -> Object {
    let shape = crs_geometry::rectangle(name, x, y, width, height);
    let result = Object::new();

    set_property(&result, &"name".into(), &shape.name.into());
    set_point_array_property(&result, "points", &shape.points);
    set_int_array_property(&result, "indices", &shape.indices);

    return result;
}