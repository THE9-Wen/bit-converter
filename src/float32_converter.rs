use crate::common_converter::ValueConverter;
use crate::fix16_converter::Fix16ToFloatConverter;
use crate::float16_converter::Float16ToFloatConverter;

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

pub struct  Float32ToFloat16Converter;

impl ValueConverter for Float32ToFloat16Converter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                String::from(format!("0x{:04X}", Self::float32_to_float16(bits)))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

impl Float32ToFloat16Converter {
    pub fn float32_to_float16(bits: u32) -> u16 {
        let sign_bit = (bits >> 31) & 0x1;
        let exponent_bits = (bits >> 23) & 0x1f;
        let fraction_bits = bits & 0x7fffff;
        ((sign_bit << 15) | (exponent_bits << 10) | fraction_bits) as u16
    }
}

pub struct Float32ToFix32Converter {
    pub(crate) bit: u32,
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
    pub(crate) bit: u32,
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
    pub(crate) bit: u32,
}

impl ValueConverter for Float32ToFixComplex16Converter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                // let img = bits & 0xffff;
                // let real = bits >> 16;
                // let img = (img * 2f32.powi(self.bit as i32)).round() as i16;
                // let real = (real * 2f32.powi(self.bit as i32)).round() as i16;
                todo!();
                // String::from(format!("{} + {}i", real, img))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

pub struct Float32ToComplexConverter;

impl ValueConverter for Float32ToComplexConverter {
    fn convert(&self, string: &str) -> String {
        let bits = u32::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                let img = (bits & 0xffff) as u16;
                let real = (bits >> 16) as u16;
                String::from(format!("{} + {}i", Float16ToFloatConverter::float16_to_float(real), Float16ToFloatConverter::float16_to_float(img)))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}