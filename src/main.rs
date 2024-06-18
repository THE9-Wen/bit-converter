use std::fs::File;
use std::io::{BufRead, BufReader, Error, stdin, Write};
use std::path::Path;
use std::str::FromStr;
use regex::Regex;
use crate::common_converter::ValueConverter;

use crate::value_converter_factory::ValueConverterFactory;
use crate::value_converter_factory::ValueType::{Float, Float32};

mod value_converter_factory;
mod float_converter;
mod float32_converter;
mod float16_converter;
mod fix32_converter;
mod fix16_converter;
mod complex_converter;
mod complex16_converter;
mod fix_complex16_converter;
mod common_converter;

fn main() -> Result<(), Error> {
    println!("***********Value Converter**********
Supported value type:
1. float32
2. float16
3. float
4. fix32
5. fix16
6. complex16
7. complex
Current converter: float32 to float
Input a number or file path to convert
Input two value types divided by a blank space to switch converter
Input \"quit\" to quit
************************************");
    let mut input = String::new();
    let mut trimmed_input;
    let buffer = stdin();
    let mut factory = ValueConverterFactory::new();
    let mut converter = factory.create("float32 float").unwrap();
    let bit_stream = Regex::new(r"^(0x)?[0-9A-Fa-f]{1,8}$").unwrap();
    let number = Regex::new(r"^[+-]?(\d+(\.\d*)?|\.\d+)$").unwrap();
    loop {
        input.clear();
        buffer.read_line(&mut input).unwrap();
        trimmed_input = input.trim();
        let converter_tmp = factory.create(trimmed_input);
        match converter_tmp {
            Some(converter_tmp) => {
                converter = converter_tmp;
            }
            None => {
                if trimmed_input == "quit" {
                    return Ok(());
                } else if bit_stream.is_match(trimmed_input) {
                    println!("{}", converter.convert(&trimmed_input[2..]));
                } else if number.is_match(trimmed_input) {
                    println!("{}", converter.convert(trimmed_input));
                } else {
                    process_file(&trimmed_input, &*converter);
                }
            }
        }
    }
}

fn process_file(path: &str, converter: &dyn ValueConverter) {
    let path = Path::new(path);
    let file = File::open(path);
    match file {
        Ok(file) => {
            let buffered = BufReader::new(file);
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_dir = path.parent().unwrap_or(".".as_ref());
            let file_extension = path.extension().unwrap_or("".as_ref()).to_str().unwrap();
            let mut file_out = File::create(Path::join(file_dir, format!("{}_out.{}", &file_name[..(file_name.len() - file_extension.len() - 1)], file_extension))).unwrap();
            for line in buffered.lines() {
                match line {
                    Ok(line) => {
                        let value;
                        if line.starts_with("0x") {
                            value = converter.convert(&line[2..]);
                        } else {
                            value = converter.convert(&line);
                        }
                        file_out.write_all(format!("{}\n", value).as_bytes()).unwrap();
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
