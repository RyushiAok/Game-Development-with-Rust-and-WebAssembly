mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Rust WASM world!!");
}

#[wasm_bindgen]
pub fn draw_triangle() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context.move_to(300., 0.);
    context.begin_path();
    context.line_to(0.0, 600.0);
    context.line_to(600.0, 600.0);
    context.line_to(300.0, 0.0);
    context.close_path();
    context.stroke();
    context.fill();
    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();
    console::log_1(&JsValue::from_str("Hello, WASM world!"));
    Ok(())
}
