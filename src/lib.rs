mod utils;

use std::iter::Map;
use wasm_bindgen::prelude::*;
use image::{GenericImageView, imageops};
use image::io::Reader;
use std::io::Cursor;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(text: &str) {
    alert(text);
}

#[wasm_bindgen]
pub fn resize_image(image_data: Vec<u8>, resize_factor: f64) -> Vec<u8> {
    // let image = Reader::open(image_url).unwrap().decode().unwrap();
    let image = Reader::new(Cursor::new(image_data))
        .with_guessed_format().unwrap().decode().unwrap();
    let (width, height) = image.dimensions();
    let new_width = (width as f64 * resize_factor) as u32;
    let new_height = (height as f64 * resize_factor) as u32;
    let resized_image = image.resize_to_fill(
        new_width,
        new_height,
        imageops::Lanczos3);

    resized_image.into_rgb8().into_iter().map(|item| {
        *item as u8
    }).collect()
}
