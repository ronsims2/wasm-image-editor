mod utils;

use wasm_bindgen::prelude::*;
use js_sys::{Map};
use image::{DynamicImage, GenericImageView, ImageOutputFormat, imageops};
use image::io::Reader;
use std::io::Cursor;
use exif;
use exif::{Exif, In, Tag, Field};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(text: &str) {
    alert(text);
}

// TODO: This would be a good function to expose, or expose a wrapped version that returns an object
fn get_orientation(exif_data: &Exif) -> i8 {
    let mut val = 0;

    let orientation_field = exif_data.get_field(Tag::Orientation, In::PRIMARY);

    match orientation_field {
        Some(orientation) => {
            match orientation.value.get_uint(0) {
                Some(orientation_val@ 1..=8) => {
                    val = orientation_val as i8;
                },
                _ => (),
            }
        },
        None => (),
    }

    return val.clone()
}

fn exif_to_list(data: &Exif) -> Map {
    let exif_fields = data.fields();
    let mut obj = Map::new();

    for f in exif_fields {

        obj.set(
            &JsValue::from(f.tag.to_string()),
            &JsValue::from(f.display_value().with_unit(data).to_string()));
    }

    obj
}

#[wasm_bindgen]
pub fn get_exif_data(image_data: Vec<u8>) -> Map {
    let exif_reader = exif::Reader::new();
    let mut image_data_buffer = Cursor::new(image_data);

    match exif_reader.read_from_container(&mut image_data_buffer) {
        Ok(exif_info) => {
            return exif_to_list(&exif_info);
        },
        Err(err) => {
            log(&err.to_string())
        }
    }
    Map::new()
}

fn rotate_from_orientation(image: DynamicImage, orientation: i8) -> DynamicImage {
    // https://magnushoff.com/articles/jpeg-orientation/
    let rotated_image = match &orientation {
        // zero means no orientation exists
        1 => image, // do noting no rotation
        2 => image, // flipped horizontally (mirrored)
        3 => {
            image.rotate180()
        }, // rotated 180 cw, fix by rotating 180 cw
        4 => image, // rotated 180 cw & flip horizontally
        5 => image, // rotated 90 & flip horizontally
        6 => {
            image.rotate90()
        }, // rotated 270 cw, fix by rotating 90 cw
        7 => image, // rotated 270 cw & flip horizontally
        8 => {
            image.rotate270()
        }, // rotated 90 cw, fix by rotating 270 cw
        _ => image,
    };

    rotated_image.clone()
}

#[wasm_bindgen]
pub fn resize_image(image_data: Vec<u8>, resize_factor: f64) -> Vec<u8> {
    // let image = Reader::open(image_url).unwrap().decode().unwrap();
    let image_data_copy = &image_data.clone();
    let mut image_data_buffer = Cursor::new(image_data_copy);
    let exif_reader = exif::Reader::new();

    // by default do nothing
    let mut orientation = 1;

    match exif_reader.read_from_container(&mut image_data_buffer) {
        Ok(exif_info) => {
            orientation = get_orientation(&exif_info);
        },
        Err(err) => {
            log(&err.to_string())
        }
    }


    log(&format!("Orientation value: {}", &orientation));

    let image = Reader::new(Cursor::new(image_data))
        .with_guessed_format().unwrap().decode().unwrap();
    let rotated_image = rotate_from_orientation(image, orientation);
    let (width, height) = rotated_image.dimensions();
    let new_width = (width as f64 * resize_factor) as u32;
    let new_height = (height as f64 * resize_factor) as u32;

    let resized_image = rotated_image.resize(
        new_width,
        new_height,
        imageops::Nearest);

    log(format!("new width: {} | old width: {} | new height: {} | old height: {}", &new_width, &width, &new_height, &height).as_str());
    let (w, h) = &resized_image.dimensions();
    log(format!("new image size: width {} | height {}", w, h).as_str());
    // These bytes don't render...
    // resized_image.into_rgb8().into_iter().map(|item| {
    //     *item as u8
    // }).collect()

    // returns pixel bytes, would need to send width x height to reconstruct using canvas
    // resized_image.into_bytes()

    // With out the cursor wrapping it, you get a warning about an unimplemented seek trait
    // https://stackoverflow.com/questions/53146982/how-does-one-pass-a-vect-to-a-function-when-the-trait-seek-is-required
    let mut image_data: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    // TODO: Add param for output type and jpeg quality
    resized_image.write_to(&mut image_data, ImageOutputFormat::Png).unwrap();

    image_data.into_inner()
}
