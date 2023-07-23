use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
// use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();


    // Your code goes here!
    // console::log_1(&JsValue::from_str("Hello world!"));

    // Ok(())
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

    /*
    context.move_to(300.0, 0.0); // top of triangle
    context.begin_path();
    context.line_to(0.0, 600.0); // bottom left of triangle
    context.line_to(600.0, 600.0); // bottom right of triangle
    context.line_to(300.0, 0.0); // top of triangle
    context.close_path();
    context.stroke();
    context.fill();
    */

    /*
    draw_triangle(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)]);

    draw_triangle(&context, [(300.0, 0.0), (150.0, 300.0), (450.0, 300.0)]);
    draw_triangle(&context, [(150.0, 300.0), (0.0, 600.0), (300.0, 600.0)]);
    draw_triangle(&context, [(450.0, 300.0), (300.0, 600.0), (600.0, 600.0)]);
    */

    sirpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 2);
    Ok(())
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3]) {
    let [top, left, right] = points;
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke()
}

fn sirpinski(context: &web_sys::CanvasRenderingContext2d,
  points: [(f64, f64); 3], depth: u8) {
    draw_triangle(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)]);
    draw_triangle(&context, [(300.0, 0.0), (150.00, 300.0), (450.0, 300.0)]);
    draw_triangle(&context, [(150.0, 300.0), (0.0, 600.0), (300.0, 600.0)]);
    draw_triangle(&context, [(450.0, 300.0), (300.0, 600.0), (600.0, 600.0)]);
}
