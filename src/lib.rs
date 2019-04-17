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
extern {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn run(s: &str) {
//     alert(s);
// }

// #[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn run(
    canvas_id: &str,
    audio_buffer: &web_sys::AudioBuffer,
    stroke_style: &JsValue,
    line_width: f64,
    step_size: usize
) {
    alert(canvas_id);
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id( canvas_id ).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let mut channel_data: [Vec<f32>; 2] = unsafe { std::mem::uninitialized() }; // Vec<Vec<f32>> = [].to_vec();

    for channel_number in  0 .. 1 {
      channel_data[channel_number] =
        audio_buffer.get_channel_data(channel_number as u32).unwrap();
    }

    let x_factor: f64 = canvas.width() as f64 / channel_data[0].len() as f64;

    let half_height: f64 = canvas.height() as f64 / 2.0;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    context.set_line_width(line_width);
    context.set_stroke_style(stroke_style);

    context.move_to(0.0, (canvas.height() as u32 / 2) as f64);

    // context
    //     .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // for (let i = 0; i < channelData[0].length; i += this.step) {
    for i in (0 .. channel_data[0].len()).step_by(step_size) {
      let mut v: f64 = 0.0;
    //   for (let c = 0; c < this.buffer.numberOfChannels; c++) {
        for c in 0 .. 1 {
    //     v += channelData[c][i];
            v += channel_data[c][i] as f64;
      }

    //   const x = i * xFactor;
    let x: f64 = i as f64 * x_factor;
    //   let y = (v / this.buffer.numberOfChannels);
    //   y = (this.height * y / 2) + (this.height / 2);
    let y: f64 = (canvas.height() as f64) * (
         (((v / 2.0) as f64 / 2.0)  as f64)
         + half_height 
        ) as f64;

    //   this.cctx.lineTo( Math.floor(x), Math.floor(y) );
      context.line_to( x, y );
    }

    // this.cctx.stroke();
    context.stroke();
}