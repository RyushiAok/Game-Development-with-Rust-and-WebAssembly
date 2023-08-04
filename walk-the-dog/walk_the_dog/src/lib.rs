mod utils;

use rand::Rng;
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

fn sierpinski(
    draw_triangle: &mut dyn (FnMut([(f64, f64); 3]) -> ()),
    points: [(f64, f64); 3],
    depth: u32,
) {
    if depth == 0 {
        draw_triangle(points);
    } else {
        let [top, left, right] = points;
        let left_mid = ((top.0 + left.0) / 2., (top.1 + left.1) / 2.);
        let right_mid = ((top.0 + right.0) / 2., (top.1 + right.1) / 2.);
        let bottom_mid = ((left.0 + right.0) / 2., (left.1 + right.1) / 2.);
        sierpinski(draw_triangle, [top, left_mid, right_mid], depth - 1);
        sierpinski(draw_triangle, [left_mid, left, bottom_mid], depth - 1);
        sierpinski(draw_triangle, [right_mid, bottom_mid, right], depth - 1);
    }
}

fn draw_triangle(
    ctx: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3],
    rgb: Option<(u32, u32, u32)>,
) {
    let [top, left, right] = points;
    ctx.move_to(top.0, top.1);
    ctx.begin_path();
    ctx.line_to(left.0, left.1);
    ctx.line_to(right.0, right.1);
    ctx.line_to(top.0, top.1);
    ctx.close_path();
    ctx.stroke();
    rgb.map(|(r, g, b)| ctx.set_fill_style(&JsValue::from_str(&format!("rgb({},{},{})", r, g, b))));
    ctx.fill();
}

fn get_canvas() -> (
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
) {
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
    return (canvas, context);
}

#[wasm_bindgen]
pub fn draw_sierpinski_triangle() -> Result<(), JsValue> {
    let (canvas, context) = get_canvas();
    let mut rng = rand::thread_rng();
    sierpinski(
        &mut |points| {
            let color = (
                rng.gen_range(0..255),
                rng.gen_range(0..255),
                rng.gen_range(0..255),
            );
            draw_triangle(&context, points, Some(color))
        },
        [
            (canvas.width() as f64 / 2., 0.),
            (0., canvas.height() as f64),
            (canvas.width() as f64, canvas.height() as f64),
        ],
        7,
    );
    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    set_panic_hook();
    console::log_1(&JsValue::from_str("Hello, WASM world!"));
    let image = web_sys::HtmlImageElement::new().unwrap();
    let (_, context) = get_canvas();
    let callback = Closure::once(|| web_sys::console::log_1(&JsValue::from_str("loaded image!")));
    image.set_src("Idle (1).png");
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    context.draw_image_with_html_image_element(&image, 0., 0.)?;
    callback.forget();
    Ok(())
}
