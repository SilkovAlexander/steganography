mod encode;

extern crate clap;
use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};
use crate::encode::encode_data;

fn parse_arguments(matches: &ArgMatches, check_data_exists: bool) -> Result<(String, String), String> {
    // Here we can unwrap without check, because options were marked as required.
    let img_path = matches.value_of("IMAGE").unwrap().to_string();
    let data_path = matches.value_of("DATA").unwrap().to_string();
    if !std::path::Path::new(&img_path).exists() {
        return Err("Image file was not found.".to_string());
    }
    if check_data_exists && !std::path::Path::new(&data_path).exists() {
        return Err("Data file was not found.".to_string());
    }
    Ok((img_path, data_path))
}

fn on_error<T>(error: T)
where
    T: std::fmt::Display
{
    println!("Error: {}", error);
    std::process::exit(1);
}

fn main() {
    let _ = main_internal()
        .map_err(|e| on_error(e));
}

fn main_internal() -> Result<(), String> {
    let matches = App::new("tonos_cli")
        .version(&*format!("{}", env!("CARGO_PKG_VERSION")))
        .author("SilkovAlexander")
        .about("Command line tool for image steganography")
        .subcommand(SubCommand::with_name("encode")
            .about("Command to encode data into the image.")
            .arg(Arg::with_name("IMAGE")
                .help("Path to the file with container image.")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("DATA")
                .help("Path to the file with data, that should be encoded.")
                .required(true)
                .takes_value(true)))
        .subcommand(SubCommand::with_name("decode")
            .about("Command to decode data from the image.")
            .arg(Arg::with_name("IMAGE")
                .help("Path to the file with container image.")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("DATA")
                .help("Path to the file, where to store the decoded data.")
                .required(true)
                .takes_value(true)))
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("decode") {
        let (img_path, data_path) = parse_arguments(matches, false)?;
        decode_data(img_path, data_path)?;
    }

    if let Some(matches) = matches.subcommand_matches("encode") {
        let (img_path, data_path) = parse_arguments(matches, true)?;
        encode_data(img_path, data_path)?;
    }
    println!("The program has finished successfully.");
    Ok(())
}


fn decode_data(_img_path: String, _data_path: String) -> Result<(), String> {

    Ok(())
}
