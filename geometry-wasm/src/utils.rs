use wasm_bindgen::JsValue;
use crs_geometry::Point;

pub fn set_property(obj: &js_sys::Object, property: &JsValue, value: &JsValue) {
    js_sys::Reflect::set(obj, property, value).ok();
}

pub fn set_point_array_property(obj: &js_sys::Object, property: &str, collection: &Vec<Point>) {
    let values = js_sys::Array::new();
    for item in collection.iter() {
        values.push(&item.x.into());
        values.push(&item.y.into());
    }
    js_sys::Reflect::set(obj, &JsValue::from(property), &values).ok();
}

pub fn set_int_array_property(obj: &js_sys::Object, property: &str, collection: &Vec<i32>) {
    let values = js_sys::Array::new();

    for item in collection.iter() {
        values.push(&JsValue::from(*item));
    }

    js_sys::Reflect::set(obj, &JsValue::from(property), &values).ok();
}