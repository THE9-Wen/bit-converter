use crate::common_converter::ValueConverter;

pub struct Fix32ToFloatConverter {
    pub(crate) bit: u32,
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

pub struct Fix32ToFloat32Converter {
    pub(crate) bit: u32
}

impl ValueConverter for Fix32ToFloat32Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

pub struct  Fix32ToFloat16Converter {
    pub(crate) bit: u32
}

impl ValueConverter for Fix32ToFloat16Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

pub struct  Fix32ToComplex16Converter {
    pub(crate) bit: u32
}

impl ValueConverter for Fix32ToComplex16Converter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

pub struct  Fix32ToComplexConverter {
    pub(crate) bit: u32
}

impl ValueConverter for Fix32ToComplexConverter {
    fn convert(&self, string: &str) -> String {
        todo!()
    }
}

