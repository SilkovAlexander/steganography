extern crate image;

use std::fs;
use image::{ColorType, DynamicImage};
use crate::helpers::{check_image_color_type, load_length, convert_bits_to_bytes};
use crate::constants::{NUMBER_OF_COLORS, LENGTH_FIELD, BITS_IN_BYTE};

pub fn decode_data(img_path: String, data_path: String) -> Result<(), String> {
    let image = image::open(img_path)
        .map_err(|e| format!("failed to open the image: {}", e))?;

    let data = load_data(image)?;
    let data = convert_bits_to_bytes(data)?;
    fs::write(data_path, data)
        .map_err(|e| format!("Failed to save the decoded data: {}", e))?;
    Ok(())
}

fn load_data(image: DynamicImage) -> Result<Vec<u8>, String> {
    let color = check_image_color_type(&image)?;

    let mut res = match color {
        ColorType::Rgba8 => {
            let pixels = image.as_rgba8()
                .ok_or("Failed to decode the image.".to_string())?
                .enumerate_pixels();
            let mut res = vec!();
            for (_, _, pixel) in pixels {
                for i in 0..NUMBER_OF_COLORS {
                    res.push(pixel.0[i] & 1);
                }
            }
            res
        },
        ColorType::Rgb8 => {
            let pixels = image.as_rgb8()
                .ok_or("Failed to decode the image.".to_string())?
                .enumerate_pixels();
            let mut res = vec!();
            for (_, _, pixel) in pixels {
                for i in 0..NUMBER_OF_COLORS {
                    res.push(pixel.0[i] & 1);
                }
            }
            res
        },
        _ => return Err("Input image has wrong color type.".to_string())
    };

    let length = load_length(&res) as usize;
    if length == 0 {
        return Err("Image doesn't contain data.".to_string());
    }
    if res.len() < LENGTH_FIELD + (length * BITS_IN_BYTE) {
        return Err("Image doesn't contain valid data".to_string());
    }
    res = res[LENGTH_FIELD..LENGTH_FIELD + (length * BITS_IN_BYTE) as usize].to_vec();

    Ok(res)
}