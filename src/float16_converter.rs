use crate::common_converter::ValueConverter;

pub struct Float16ToFloatConverter;

impl ValueConverter for Float16ToFloatConverter {
    fn convert(&self, string: &str) -> String {
        let bits = u16::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                String::from(format!("{}", Float16ToFloatConverter::float16_to_float(bits)))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

impl Float16ToFloatConverter {
    pub fn float16_to_float(bits: u16) -> f32 {
        f32::from_bits(Float16ToFloat32Converter::float16_to_float32(bits))
    }
}

pub struct Float16ToFloat32Converter;

impl ValueConverter for Float16ToFloat32Converter {
    fn convert(&self, string: &str) -> String {
        let bits = u16::from_str_radix(string, 16);
        match bits {
            Ok(bits) => {
                String::from(format!("{}", Float16ToFloat32Converter::float16_to_float32(bits)))
            }
            Err(_) => {
                println!("Error when parse line: {}", string);
                String::from("NAN")
            }
        }
    }
}

impl Float16ToFloat32Converter {
    pub fn float16_to_float32(bits: u16) -> u32 {
        let sign_bit = ((bits >> 15) & 0x1) as u32;
        let exponent_bits = ((bits >> 10) & 0x1f) as u32;
        let fraction_bits = (bits & 0x3f) as u32;
        (sign_bit << 31) | (exponent_bits << 23) | fraction_bits
    }
}

pub struct Float16ToFix32Converter {
    pub(crate) bit: u32,
}

impl ValueConverter for Float16ToFix32Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

pub struct Float16ToFix16Converter {
    pub(crate) bit: u32,
}

impl ValueConverter for Float16ToFix16Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

pub struct Float16ToFixComplex16Converter {
    pub(crate) bit: u32,
}

impl ValueConverter for Float16ToFixComplex16Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}