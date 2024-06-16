use crate::common_converter::ValueConverter;

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
        todo!()
    }
}