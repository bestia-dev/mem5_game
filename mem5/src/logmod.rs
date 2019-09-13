//! logmod.rs - logging in wasm

use web_sys::console;
use wasm_bindgen::prelude::*;

///simple console write with a string
pub fn log1_str(x: &str) {
    console::log_1(&JsValue::from_str(x));
}
/*
///simple console write with JsValue
pub fn log1_jsvalue(x: &JsValue) {
    console::log_1(x);
}
*/
