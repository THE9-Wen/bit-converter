use std::fs::File;
use std::io::{BufRead, BufReader, Error, stdin, Write};
use std::path::Path;
use std::str::FromStr;
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
    let mut input = String::new();
    let mut trimmed_input;
    let buffer = stdin();
    let mut factory = ValueConverterFactory::new();
    let mut converter = factory.create(Float32 as usize, Float as usize).unwrap();
    loop {
        input.clear();
        buffer.read_line(&mut input).unwrap();
        trimmed_input = input.trim();
        let converter_tmp = factory.create(Float32 as usize, Float as usize);
        match converter_tmp {
            Some(converter_tmp) => {
                converter = converter_tmp;
            }
            None => {
                if trimmed_input == "exit" {
                    return Ok(());
                } else if trimmed_input.starts_with("0x") {
                    println!("{}", converter.convert(&trimmed_input[2..]));
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
