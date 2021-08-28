use wasm_bindgen::JsValue;
use crs_geometry::{Point, BoundingBox};

pub fn set_property(obj: &js_sys::Object, property: &str, value: &JsValue) {
    js_sys::Reflect::set(obj, &JsValue::from(property), value).ok();
}

pub fn set_point_array_property(obj: &js_sys::Object, property: &str, collection: &Vec<Point>) {
    let values = js_sys::Array::new();
    for item in collection.iter() {
        values.push(&item.x.into());
        values.push(&item.y.into());
    }

    set_property(obj, property, &values);
}

pub fn set_int_array_property(obj: &js_sys::Object, property: &str, collection: &Vec<i32>) {
    let values = js_sys::Array::new();

    for item in collection.iter() {
        values.push(&JsValue::from(*item));
    }

    set_property(obj, property, &values);
}

pub fn set_point_property(obj: &js_sys::Object, property: &str, point: &Point) {
    let origin = js_sys::Object::new();
    set_property(&origin, "x", &point.x.into());
    set_property(&origin, "y", &point.y.into());
    set_property(obj, property, &origin);
}

pub fn set_aabb_property(obj: &js_sys::Object, property: &str, aabb: &BoundingBox) {
    let aabb_value = js_sys::Object::new();
    set_property(&aabb_value, "min_x", &JsValue::from(aabb.min_x));
    set_property(&aabb_value, "max_x", &JsValue::from(aabb.max_x));
    set_property(&aabb_value, "min_y", &JsValue::from(aabb.min_y));
    set_property(&aabb_value, "max_y", &JsValue::from(aabb.max_y));
    set_property(&aabb_value, "width", &JsValue::from(aabb.width));
    set_property(&aabb_value, "height", &JsValue::from(aabb.height));
    set_property(obj, property, &aabb_value);
}