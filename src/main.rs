use std::fs::File;
use std::{env, io};
use std::fmt::format;
use std::io::{BufRead, BufReader, Error, stdin, Write};
use std::num::{ParseFloatError, ParseIntError};
use std::path::Path;
use std::str::FromStr;

pub trait ValueConverter {
    fn convert(&self, string: &str) -> String;
}

pub struct Fix32ToFloatConverter {
    bit: u32,
}

impl ValueConverter for Fix32ToFloatConverter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                String::from(format!("{}", (bits as f64 / 2f64.powi(self.bit as i32)) as f32))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct Float32ToFloatConverter;

impl ValueConverter for Float32ToFloatConverter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                String::from(format!("{}", f32::from_bits(bits)))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct Float16ToFloatConverter;

impl ValueConverter for Float16ToFloatConverter {
    fn convert(&self, string: &str) -> String {
        let bits = u16::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                String::from(format!("{}", f32::from_bits(bits as u32)))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct Fix16ToFloatConverter {
    bit: u32,
}

impl ValueConverter for Fix16ToFloatConverter {
    fn convert(&self, string: &str) -> String {
        let bits = u16::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                String::from(format!("{}", bits as f32 / (2f32.powi(self.bit as i32))))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct FloatToFloat32Converter;

impl ValueConverter for FloatToFloat32Converter {
    fn convert(&self, string: &str) -> String {
        let value = f32::from_str(string);
        match value {
            Ok(value) => {
                String::from(format!("0x{:08X}", value.to_bits()))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct FloatToFloat16Converter;

impl ValueConverter for FloatToFloat16Converter {
    fn convert(&self, string: &str) -> String {
        let value = f32::from_str(string);
        match value {
            Ok(value) => {
                String::from(format!("0x{:04X}", value.to_bits()))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct FloatToFix16Converter {
    bit: u32,
}

impl ValueConverter for FloatToFix16Converter {
    fn convert(&self, string: &str) -> String {
        let value = f32::from_str(string);
        match value {
            Ok(value) => {
                let value = (value * 2f32.powi(self.bit as i32)).round() as i32;
                String::from(format!("0x{:04X}", value))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct FloatToFix32Converter {
    bit: u32,
}

impl ValueConverter for FloatToFix32Converter {
    fn convert(&self, string: &str) -> String {
        let value = f64::from_str(string);
        match value {
            Ok(value) => {
                let value = (value * 2f64.powi(self.bit as i32)).round() as i32;
                String::from(format!("0x{:08X}", value))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

struct ValueConverterFactory;

fn read_integer_form_stdin() -> u32 {
    println!("Please enter the bit width of the fractional part of the fixed-point:");
    let buffer = stdin();
    let mut string = String::new();
    buffer.read_line(&mut string).unwrap();
    match string.parse::<u32>() {
        Ok(value) => {
            value
        }
        Err(_) => {
            println!("Invalid bit width");
            read_integer_form_stdin()
        }
    }
}

impl ValueConverterFactory {
    fn create(&self, name: &str) -> Option<Box<dyn ValueConverter>> {
        let mut bit;
        match name {
            "fix32_to_float" => {
                bit = read_integer_form_stdin();
                Some(Box::new(Fix32ToFloatConverter {
                    bit: read_integer_form_stdin(),
                }))
            }
            "fix16_to_float" => {
                bit = read_integer_form_stdin();
                Some(Box::new(Fix16ToFloatConverter {
                    bit,
                }))
            }
            "float32_to_float" => {
                Some(Box::new(Float32ToFloatConverter))
            }
            "float16_to_float" => {
                Some(Box::new(Float16ToFloatConverter))
            }
            "float_to_float32" => {
                Some(Box::new(FloatToFloat32Converter))
            }
            "float_to_float16" => {
                Some(Box::new(FloatToFloat16Converter))
            }
            "float_to_fix16" => {
                bit = read_integer_form_stdin();
                Some(Box::new(FloatToFix16Converter {
                    bit,
                }))
            }
            "float_to_fix32" => {
                bit = read_integer_form_stdin();
                Some(Box::new(FloatToFix32Converter {
                    bit,
                }))
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
