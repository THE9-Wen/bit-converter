use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::num::ParseIntError;

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

struct ValueConverterFactory {
    cur_factory: dyn ValueConverter,
}

impl ValueConverterFactory {
    fn create(&mut self, name: &str) -> &dyn ValueConverter {
        match name {
            "float32_to_float" => {
                self.cur_factory = Fix32ToFloatConverter{
                    bit: 0,
                };
                &self.cur_factory
            },
            &_ => {
                &self.cur_factory
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let path = "/Users/wenhao/RustroverProjects/tools/src/lines.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    process_file(buffered);
    Ok(())
}

fn process_file(buffered: BufReader<File>) {
    for line in buffered.lines() {
        match line {
            Ok(value) => {
                let bits;
                if value.starts_with("0x") {
                    bits = u32::from_str_radix(&value[2..], 16);
                } else {
                    bits = u32::from_str_radix(&value, 16);
                }
                match bits {
                    Ok(bits) => {
                        let float_val = f32::from_bits(bits);
                        println!("{}", float_val);
                    }
                    Err(_e) => {
                        println!("Fail to parse bit in line: {}", value);
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}
