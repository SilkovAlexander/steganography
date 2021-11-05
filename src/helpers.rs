use crate::constants::*;
use image::{DynamicImage, GenericImageView};
use crate::constants::{LENGTH_FIELD, BITS_IN_BYTE};

fn convert_length(length: u32) -> Vec<u8> {
    let mut res = vec!();
    for i in (0..LENGTH_FIELD).rev() {
        res.push(
            if (length & (1 << i)) != 0 {
                1
            } else {
                0
            }
        )
    }
    res
}

pub fn load_length(data: &Vec<u8>) -> u32 {
    if data.len() < LENGTH_FIELD as usize {
        return 0;
    }

    let mut res: u32 = 0;

    for i in 0..LENGTH_FIELD {
        res = (data[i] as u32) + (res << 1);
    }
    res
}

pub fn convert_bits_to_bytes(data_bits: Vec<u8>) -> Result<Vec<u8>, String> {
    if data_bits.len() % 8 != 0 {
        return Err("Data bits vector has wrong length".to_string());
    }
    let mut res = vec!();
    let mut bit = data_bits.iter();
    for _ in 0..(data_bits.len()/BITS_IN_BYTE) {
        let mut tmp = 0;
        for i in 0..BITS_IN_BYTE {
            tmp = bit.next().unwrap_or(&0) + (tmp << 1);
        }
        res.push(tmp);
    }
    Ok(res)
}

pub fn convert_bytes_to_bits_with_length(data_bytes: Vec<u8>) -> Vec<u8> {
    let data_length = data_bytes.len() as u32;
    let mut res= convert_length(data_length);
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
    let mut data_bits = data_bits
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();
    res.append( &mut data_bits);
    res

}

pub fn check_image_parameters(image: &DynamicImage, data_len: usize) -> Result<(), String> {
    check_image_color_type(image)?;
    let img_size = image.dimensions();
    let img_storage_volume = (img_size.0 * img_size.1 * NUMBER_OF_COLORS as u32) as usize;
    if img_storage_volume < (data_len + LENGTH_FIELD) {
        return Err("Image buffer is not great enough to accommodate the data buffer.".to_string());
    }
    Ok(())
}

pub fn check_image_color_type(image: &DynamicImage) ->Result<(), String> {
    if (image.color().channel_count() as usize) < NUMBER_OF_COLORS {
        return Err("Image is meant to have at least 3 color channels.".to_string());
    }
    Ok(())
}
