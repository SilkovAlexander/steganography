extern crate image;

use std::fs;
use image::{DynamicImage};
use crate::helpers::{check_image_parameters, check_image_color_type, load_length, convert_bits_to_bytes};
use crate::constants::NUMBER_OF_COLORS;

pub fn decode_data(img_path: String, data_path: String) -> Result<(), String> {
    let image = image::open(img_path)
        .map_err(|e| format!("failed to open the image: {}", e))?;

    check_image_color_type(&image)?;
    let data = load_data(image)?;
    let data = convert_bits_to_bytes(data)?;
    fs::write(data_path, data)
        .map_err(|e| format!("Failed to save the decoded data: {}", e))?;
    Ok(())
}

fn load_data(image: DynamicImage) -> Result<Vec<u8>, String> {
    let mut res = vec!();

    for (_, _, pixel) in image.as_rgba8()
        .ok_or("Failed to decode the image.".to_string())?
        .enumerate_pixels() {
        for i in 0..NUMBER_OF_COLORS {
            res.push(pixel.0[i] & 1);
        }
    }

    let length = load_length(&res);
    if length == 0 {
        return Err("Image doesn't contain data.".to_string());
    }
    res = res[32..32 + (length * 8) as usize].to_vec();

    Ok(res)
}