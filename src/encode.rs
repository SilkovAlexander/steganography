use std::fs;

pub fn encode_data(_img_path: String, data_path: String) -> Result<(), String> {
    let data_bytes = fs::read(&data_path)
        .map_err(|e| format!("Failed to read file {}: {}", data_path, e))?;
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
    let data_bits = data_bits
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();

    Ok(())
}
