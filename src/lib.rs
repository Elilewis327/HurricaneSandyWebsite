use wasm_bindgen::prelude::*;
use std::time::*;
use console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
extern "C"{

#[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// entry point into the program
#[wasm_bindgen]
pub fn main() {

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut now: Instant;
    let mut x = 0.0f32;
    context.set_fill_style(&"blue".into());

    loop {
        now = Instant::now();
        draw(&context, &now, &mut x);
        if x > 500.0 {
            return;
        }
    }
}


fn draw(ctx: &web_sys::CanvasRenderingContext2d, dx: &Instant, x: &mut f32){
    if Duration::from_secs(4) < dx.elapsed() {
        return;
    }

    *x = *x + 5.0;

    log(&(*x.to_string()));

    ctx.clear_rect(0.0, 0.0, 500.0, 500.0);
    ctx.fill_rect((*x).into(), 0.0, 10.0, 10.0);

}