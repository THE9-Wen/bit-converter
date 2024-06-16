use crate::common_converter::ValueConverter;

pub struct Fix16ToFloatConverter {
    pub(crate) bit: u32,
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

pub struct Fix16ToFloat16Converter {
    pub(crate) bit: u32,
}

impl ValueConverter for Fix16ToFloat16Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

pub struct Fix16ToFloat32Converter {
    pub(crate) bit: u32,
}

impl ValueConverter for Fix16ToFloat32Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}