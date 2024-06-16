use crate::common_converter::ValueConverter;

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

pub struct Float16ToFloat32Converter;

impl ValueConverter for Float16ToFloat32Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
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