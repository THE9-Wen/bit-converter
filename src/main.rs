use std::fs::File;
use std::{env, io};
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;

pub trait ValueConverter {
    fn convert(&self, string: &str) -> Option<f32>;
}

pub struct Fix32ToFloatConverter {
    bit: u32,
}

impl ValueConverter for Fix32ToFloatConverter {
    fn convert(&self, string: &str) -> Option<f32> {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                Some((bits as f64 / (2f64.powi(self.bit as i32))) as f32)
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                None
            }
        }
    }
}

pub struct Float32ToFloatConverter;

impl ValueConverter for Float32ToFloatConverter {
    fn convert(&self, string: &str) -> Option<f32> {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                Some(f32::from_bits(bits))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                None
            }
        }
    }
}

pub struct Float16ToFloatConverter;

impl ValueConverter for Float16ToFloatConverter {
    fn convert(&self, string: &str) -> Option<f32> {
        let bits = u16::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                Some(f32::from_bits(bits as u32))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                None
            }
        }
    }
}

pub struct Fix16ToFloatConverter {
    bit: u32,
}

impl ValueConverter for Fix16ToFloatConverter {
    fn convert(&self, string: &str) -> Option<f32> {
        let bits = u16::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                Some(bits as f32 / (2f32.powi(self.bit as i32)))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                None
            }
        }
    }
}

struct ValueConverterFactory;

impl ValueConverterFactory {
    fn create(&self, name: &str) -> Option<&dyn ValueConverter> {
        match name {
            "fix32_to_float" => {
                Some(&Fix32ToFloatConverter {
                    bit: 17,
                })
            },
            "fix16_to_float" => {
                Some(&Fix16ToFloatConverter {
                    bit: 9,
                })
            },
            "float32_to_float" => {
                Some(&Float32ToFloatConverter)
            },
            "float16_to_float" => {
                Some(&Float16ToFloatConverter)
            }
            &_ => {
                None
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    let mut trimmed_input;
    let buffer = io::stdin();
    let mut factory = ValueConverterFactory;
    let converter = factory.create("float32_to_float").unwrap();
    loop {
        input.clear();
        buffer.read_line(&mut input).unwrap();
        trimmed_input = input.trim();
        let converter_tmp = factory.create(&trimmed_input);
        match converter_tmp {
            Some(converter) => {},
            None => {
                if trimmed_input == "exit" {
                    return Ok(());
                } else {
                    process_file(&trimmed_input, converter);
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
                        file_out.write_all(format!("{}\n", value.unwrap_or(f32::NAN)).as_bytes()).unwrap();
                    }
                    Err(e) => println!("{}", e),
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}
