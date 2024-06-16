use std::str::FromStr;
use crate::common_converter::ValueConverter;

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
    pub(crate) bit: u32,
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
    pub(crate) bit: u32,
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

pub struct FloatToFixComplex16Converter {
    pub(crate) bit: u32,
}

impl ValueConverter for FloatToFixComplex16Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

pub struct FloatToComplex16Converter;

impl ValueConverter for FloatToComplex16Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

pub struct FloatToComplexConverter;

impl ValueConverter for FloatToComplexConverter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}