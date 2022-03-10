extern crate image;

use std::fs;
use image::{DynamicImage, ColorType};
use crate::helpers::{
    convert_bytes_to_bits_with_length,
    check_image_parameters,
};
use crate::constants::NUMBER_OF_COLORS;

pub fn encode_data(img_path: String, data_path: String, output_path: String) -> Result<(), String> {
    let data_bytes = fs::read(&data_path)
        .map_err(|e| format!("failed to read file {}: {}", data_path, e))?;
    check_data(&data_bytes)?;

    let data_bits = convert_bytes_to_bits_with_length(data_bytes);
    let image = image::open(img_path)
        .map_err(|e| format!("failed to open the image: {}", e))?;

    let encoded_image = encapsulate_data(data_bits, image)?;
    encoded_image.save(output_path)
        .map_err(|e| format!("Failed to save the result image: {}", e))?;

    Ok(())
}

fn check_data(data_bytes: &Vec<u8>) -> Result<(), String> {
    let max_data_size = u32::MAX;
    if data_bytes.len() * 8 > max_data_size as usize {
        return Err(format!("Data size is too great. Please use data chunks not greater then {} bits", max_data_size));
    }
    Ok(())
}

fn encapsulate_data(data_bits: Vec<u8>, image: DynamicImage) -> Result<DynamicImage, String> {
    let color = check_image_parameters(&image, data_bits.len())?;
    let mut iter = data_bits.iter();
    let mut n_image = image;
    // TODO: separate rgba,rgb encoding or convert all to rgb8
    match color {
        ColorType::Rgba8 => {
            let pixels = n_image.as_mut_rgba8()
                .ok_or("Failed to decode the image.".to_string())?
                .enumerate_pixels_mut();
            for (_, _, pixel) in pixels
            {
                for i in 0..NUMBER_OF_COLORS {
                    pixel.0[i] = (pixel.0[i] & 0xFE) + iter.next().unwrap_or(&0);
                }
            }
        },
        ColorType::Rgb8 => {
            let pixels = n_image.as_mut_rgb8()
                .ok_or("Failed to decode the image.".to_string())?
                .enumerate_pixels_mut();
            for (_, _, pixel) in pixels
            {
                for i in 0..NUMBER_OF_COLORS {
                    pixel.0[i] = (pixel.0[i] & 0xFE) + iter.next().unwrap_or(&0);
                }
            }
        },
        _ => return Err("Input image has wrong color type.".to_string())
    };

    Ok(n_image)
}
