use crate::value_converter_factory::ValueType;
use crate::value_converter_factory::ValueType::{Complex, Float};

pub trait ValueConverter {
    fn convert(&self, string: &str) -> String;

    fn is_self_converter(&self) -> bool {
        false
    }
}

pub struct SelfConverter {
    pub(crate) value_type: i32
}

impl ValueConverter for SelfConverter {
    fn convert(&self, string: &str) -> String {
        if self.value_type == Float as i32 && self.value_type == Complex as i32 {
            format!("{}", string)
        } else {
            format!("0x{}", string)
        }
    }

    fn is_self_converter(&self) -> bool {
        true
    }
}