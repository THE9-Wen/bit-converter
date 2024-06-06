mod value_converter;

use std::fs::File;
use std::{env, io};
use std::fmt::format;
use std::io::{BufRead, BufReader, Error, stdin, Write};
use std::num::{ParseFloatError, ParseIntError};
use std::path::Path;
use std::str::FromStr;
use crate::value_converter::{ValueConverter, ValueConverterFactory};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let mut trimmed_input;
    let buffer = stdin();
    let mut factory = ValueConverterFactory;
    let mut converter = factory.create("float32_to_float").unwrap();
    loop {
        input.clear();
        buffer.read_line(&mut input).unwrap();
        trimmed_input = input.trim();
        let converter_tmp = factory.create(&trimmed_input);
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
