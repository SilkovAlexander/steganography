extern crate image;

use std::fs;
use image::{DynamicImage};
use crate::helpers::{
    convert_bytes_to_bits_with_length,
    check_image_parameters,
};
use crate::constants::NUMBER_OF_COLORS;

pub fn encode_data(img_path: String, data_path: String, _output_path: Option<String>) -> Result<(), String> {
    let data_bytes = fs::read(&data_path)
        .map_err(|e| format!("failed to read file {}: {}", data_path, e))?;
    check_data(&data_bytes)?;

    let data_bits = convert_bytes_to_bits_with_length(data_bytes);
    let image = image::open(img_path)
        .map_err(|e| format!("failed to open the image: {}", e))?;

    check_image_parameters(&image, data_bits.len())?;
    encapsulate_data(data_bits, image)?;

    Ok(())
}

fn check_data(data_bytes: &Vec<u8>) -> Result<(), String> {
    let max_data_size = u32::MAX;
    if data_bytes.len() * 8 > max_data_size as usize {
        return Err(format!("Data size is too great. Please use data chunks not greater then {} bits", max_data_size));
    }
    Ok(())
}

fn encapsulate_data(data_bits: Vec<u8>, image: DynamicImage) -> Result<(), String> {
    let mut iter = data_bits.iter();
    // TODO: separate rgba,rgb encoding or convert all to rgb8
    let mut n_image = image;
    for (_, _, pixel) in n_image.as_mut_rgba8()
        .ok_or("Failed to decode the image.".to_string())?
        .enumerate_pixels_mut() {
        for i in 0..NUMBER_OF_COLORS {
            pixel.0[i] = (pixel.0[i] & 0xFE) + iter.next().unwrap_or(&0);
        }
    }
    // TODO: return updated image and save it to specified path in a function
    n_image.save("tests/resources/result.png")
        .map_err(|e| format!("Failed to save the result image: {}", e))?;
    Ok(())
}
