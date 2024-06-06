use std::any::Any;
use std::io::stdin;
use std::str::FromStr;
use crate::value_converter::ValueType::{Complex, Complex16, Fix16, Fix32, FixComplex16, Float, Float16, Float32, ValueTypeNum};

pub trait ValueConverter {
    fn convert(&self, string: &str) -> String;
}

pub struct SelfConverter {
    value_type: i32
}

impl ValueConverter for SelfConverter {
    fn convert(&self, string: &str) -> String {
        if self.value_type == Float as i32 && self.value_type == Complex as i32 {
            format!("{}", string)
        } else {
            format!("0x{}", string)
        }
    }
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

pub struct  Float32ToFloat16Converter;

impl ValueConverter for Float32ToFloat16Converter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                let sign_bit = (bits >> 31) & 0x1;
                let exponent_bits = (bits >> 23) & 0x1f;
                let fraction_bits = bits & 0x7fffff;
                let num_16bit = (sign_bit << 15) | (exponent_bits << 10) | fraction_bits;
                String::from(format!("0x{:04X}", num_16bit))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct Float32ToFix32Converter {
    bit: u32,
}

impl ValueConverter for Float32ToFix32Converter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                let value = f32::from_bits(bits);
                let value = (value as f64 * 2f64.powi(self.bit as i32)).round() as i32;
                String::from(format!("0x{:08X}", value))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct Float32ToFix16Converter {
    bit: u32,
}

impl ValueConverter for Float32ToFix16Converter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                let value = f32::from_bits(bits);
                let value = (value * 2f32.powi(self.bit as i32)).round() as i16;
                String::from(format!("0x{:04X}", value))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct Float32ToFixComplex16Converter {
    bit: u32,
}

impl ValueConverter for Float32ToFixComplex16Converter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                let img = bits & 0xffff;
                let real = bits >> 16;
                let img = (img * 2f32.powi(self.bit as i32)).round() as i16;
                let real = (real * 2f32.powi(self.bit as i32)).round() as i16;
                todo!();
                String::from(format!("{} + {}i", real, img))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct ValueConverterFactory {
    converters: Box<[[dyn ValueConverter; ValueTypeNum as usize]; ValueTypeNum as usize]>,
}

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
    pub fn new() -> Self {
        let mut value_converters = Box::new([[None; ValueTypeNum as usize]; ValueTypeNum as usize]);
        value_converters[Float32][Float32] = Some(SelfConverter{ value_type: Float32 as i32 });
        value_converters[Float32][Float16] = Some(Float32ToFloat16Converter);
        value_converters[Float32][Float] = Some(Float32ToFloatConverter);
        value_converters[Float32][Fix32] = Some(Float32ToFix32Converter{ bit: 0 });
        value_converters[Float32][Fix16] = Some(Float32ToFix16Converter{ bit: 0 });
        value_converters[Float32][FixComplex16] = Some(Float32ToFixComplex16Converter{ bit: 0 });
        value_converters[Float32][Complex16] = Some(SelfConverter{ value_type: Float32 as i32});
        value_converters[Float32][Complex] = Some(Float32ToFixComplex16Converter{ bit: 0 });
        Self {
            converters: value_converters,
        }
    }
    pub(crate) fn create(&self, name: &str) -> Option<Box<dyn ValueConverter>> {
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

pub enum ValueType {
    Float32 = 0,
    Float16,
    Float,
    Fix32,
    Fix16,
    FixComplex16,
    Complex16,
    Complex,
    ValueTypeNum
}

impl ValueType {
    pub fn get_value_type(string: &str) -> Self {
        match string {
            "float32" => Float32,
            "float16" => ValueType::Float16,
            "float" => Float,
            "fix32" => ValueType::Fix32,
            "fix16" => ValueType::Fix16,
            "complex" => ValueType::Complex16,
            "fixcomplex16" => ValueType::FixComplex16,
            &_ => ValueTypeNum,
        }
    }
}