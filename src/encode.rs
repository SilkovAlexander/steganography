extern crate image;

use std::fs;
use image::{DynamicImage, GenericImage, GenericImageView};
use crate::constants::*;
use self::image::Pixel;

pub fn encode_data(img_path: String, data_path: String, _output_path: Option<String>) -> Result<(), String> {
    let data_bytes = fs::read(&data_path)
        .map_err(|e| format!("failed to read file {}: {}", data_path, e))?;
    check_data(&data_bytes)?;

    let data_bits = convert_bytes_to_bits(data_bytes);
    let image = image::open(img_path)
        .map_err(|e| format!("failed to open the image: {}", e))?;

    check_image_parameters(&image, data_bits.len())?;
    encapsulate_data(data_bits, image)?;

    Ok(())
}

fn convert_bytes_to_bits(data_bytes: Vec<u8>) -> Vec<u8> {
    let mut data_bits = vec!();
    data_bytes
        .iter()
        .for_each(|byte| {
            let mut bits:Vec<u8> = vec!();
            for i in (0..8).rev() {
                bits.push(if byte & (1 << i) == 0 { 0 } else { 1 });
            }
            data_bits.push(bits);
        });
    data_bits
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>()
}

fn check_data(data_bytes: &Vec<u8>) -> Result<(), String> {
    let max_data_size = u32::MAX;
    if data_bytes.len() * 8 > max_data_size as usize {
        return Err(format!("Data size is too great. Please use data chunks not greater then {} bits", max_data_size));
    }
    Ok(())
}

fn check_image_parameters(image: &DynamicImage, data_len: usize) -> Result<(), String> {
    if image.color().channel_count() < NUMBER_OF_COLORS {
        return Err("Image is meant to have at least 3 color channels.".to_string());
    }
    let img_size = image.dimensions();
    let img_storage_volume = (img_size.0 * img_size.1 * NUMBER_OF_COLORS as u32) as usize;
    if img_storage_volume < (data_len + LENGTH_FIELD as usize) {
        return Err("Image buffer is not great enough to accommodate the data buffer.".to_string());
    }
    Ok(())
}

fn encapsulate_data(data_bits: Vec<u8>, image: DynamicImage) -> Result<(), String> {
    // let mut img_buf  = image.clone().as_mut_rgb8()
    //     .ok_or("Failed to decode the image.".to_string())?
    //     .enumerate_pixels_mut();
    let mut iter = data_bits.iter();

    for (_, _, pixel) in image.clone().as_mut_rgb8()
        .ok_or("Failed to decode the image.".to_string())?
        .enumerate_pixels_mut() {
        println!("{:?}", pixel);
        for i in 0..3 {
            pixel.0[i] = (pixel.0[i] & 0xFE) + iter.next().unwrap_or(&0);
        }
        println!("{:?}", pixel);
    }
    Ok(())
}
