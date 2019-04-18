extern crate web_sys;

mod utils;
use std::f64;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[wasm_bindgen(start)]

#[wasm_bindgen]
pub fn render(
    canvas: web_sys::HtmlCanvasElement,
    audio_buffer: &web_sys::AudioBuffer,
    stroke_style: &JsValue,
    line_width: f64,
    step_size: usize,
) {
    web_sys::console::log_1(&"rust got canvas el".into());

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let mut channel_data: [Vec<f32>; 2] = unsafe { std::mem::uninitialized() }; // Vec<Vec<f32>> = [].to_vec();

    for channel_number in 0..1 {
        channel_data[channel_number] = audio_buffer
            .get_channel_data(channel_number as u32)
            .unwrap();
    }

    let x_factor: f64 = canvas.width() as f64 / channel_data[0].len() as f64;

    let half_height: f64 = canvas.height() as f64 / 2.0;

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.begin_path();
    ctx.set_line_width(line_width);
    ctx.set_stroke_style(stroke_style);

    ctx.move_to(0.0, (canvas.height() as u32 / 2) as f64);

    let mut last_x: u64 = 0;
    let mut last_y: u64 = 0;

    for i in (0..channel_data[0].len()).step_by(step_size) {
        let mut v: f64 = 0.0;
        for c in 0..1 {
            v += channel_data[c][i] as f64;
        }
        v = v / 2.0;

        let x: f64 = i as f64 * x_factor;

        let y: f64 = half_height + (v * canvas.height() as f64);

        if x as u64 != last_x && y as u64 != last_y {
            ctx.line_to(x, y);
            last_x = x as u64;
            last_y = y as u64;
        }

        // web_sys::console::log_2(&x.into(), &y.into());
        // web_sys::console::log_1(&v.into());
    }

    ctx.stroke();
    web_sys::console::log_1(&"rust done".into());
}
