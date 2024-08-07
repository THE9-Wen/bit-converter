use std::fmt::{Display, Formatter};

use crate::common_converter::{SelfConverter, ValueConverter};
use crate::complex16_converter::Complex16ToComplexConverter;
use crate::complex_converter::{ComplexToComplex16Converter, ComplexToFloat32Converter};
use crate::fix16_converter::{Fix16ToFloat16Converter, Fix16ToFloatConverter};
use crate::fix32_converter::{Fix32ToFloat32Converter, Fix32ToFloatConverter};
use crate::float16_converter::{
    Float16ToFix16Converter, Float16ToFix32Converter, Float16ToFloat32Converter,
    Float16ToFloatConverter,
};
use crate::float32_converter::{
    Float32ToComplexConverter, Float32ToFix16Converter, Float32ToFix32Converter,
    Float32ToFloat16Converter, Float32ToFloatConverter,
};
use crate::float_converter::{
    FloatToFix16Converter, FloatToFix32Converter, FloatToFloat16Converter, FloatToFloat32Converter,
};
use crate::value_converter_factory::ValueType::{
    Complex, Complex16, Fix16, Fix32, Float, Float16, Float32, ValueTypeNum,
};

pub trait ValueConverterFactory {
    fn create(dst: &ValueType, src_bit: u32, dst_bit: u32) -> Box<dyn ValueConverter>;

    fn check(dst: &ValueType) -> bool;
}

pub struct FloatConverterFactory;

impl ValueConverterFactory for FloatConverterFactory {
    fn create(dst: &ValueType, src_bit: u32, dst_bit: u32) -> Box<dyn ValueConverter> {
        match dst {
            Float32 => Box::new(FloatToFloat32Converter),
            Float16 => Box::new(FloatToFloat16Converter),
            Complex16 => Box::new(FloatToFix32Converter { bit: 0 }),
            Complex => Box::new(FloatToFix16Converter { bit: 0 }),
            Fix32 => Box::new(FloatToFix32Converter { bit: dst_bit }),
            Fix16 => Box::new(FloatToFix16Converter { bit: dst_bit }),
            _ => Box::new(SelfConverter {
                value_type: Float as i32,
            }),
        }
    }

    fn check(dst: &ValueType) -> bool {
        [Float32, Float16, Complex16, Complex, Fix32, Fix16].contains(dst)
    }
}

pub struct Float32ConverterFactory;

impl ValueConverterFactory for Float32ConverterFactory {
    fn create(dst: &ValueType, src_bit: u32, dst_bit: u32) -> Box<dyn ValueConverter> {
        match dst {
            Float16 => Box::new(Float32ToFloat16Converter),
            Float => Box::new(Float32ToFloatConverter),
            Complex => Box::new(Float32ToComplexConverter),
            Fix32 => Box::new(Float32ToFix32Converter { bit: dst_bit }),
            Fix16 => Box::new(Float32ToFix16Converter { bit: dst_bit }),
            _ => Box::new(SelfConverter {
                value_type: Float32 as i32,
            }),
        }
    }

    fn check(dst: &ValueType) -> bool {
        [Float16, Float, Complex, Fix32, Fix16].contains(dst)
    }
}

pub struct Float16ConverterFactory;

impl ValueConverterFactory for Float16ConverterFactory {
    fn create(dst: &ValueType, src_bit: u32, dst_bit: u32) -> Box<dyn ValueConverter> {
        match dst {
            Float32 => Box::new(Float16ToFloat32Converter),
            Float => Box::new(Float16ToFloatConverter),
            Fix32 => Box::new(Float16ToFix32Converter { bit: dst_bit }),
            Fix16 => Box::new(Float16ToFix16Converter { bit: dst_bit }),
            _ => Box::new(SelfConverter {
                value_type: Float16 as i32,
            }),
        }
    }

    fn check(dst: &ValueType) -> bool {
        [Float32, Float, Fix32, Fix16].contains(dst)
    }
}

pub struct Fix32ConverterFactory;

impl ValueConverterFactory for Fix32ConverterFactory {
    fn create(dst: &ValueType, src_bit: u32, dst_bit: u32) -> Box<dyn ValueConverter> {
        match dst {
            Float32 => Box::new(Fix32ToFloat32Converter { bit: src_bit }),
            Float => Box::new(Fix32ToFloatConverter { bit: src_bit }),
            _ => Box::new(SelfConverter {
                value_type: Fix32 as i32,
            }),
        }
    }

    fn check(dst: &ValueType) -> bool {
        [Float32, Float].contains(dst)
    }
}

pub struct Fix16ConverterFactory;

impl ValueConverterFactory for Fix16ConverterFactory {
    fn create(dst: &ValueType, src_bit: u32, dst_bit: u32) -> Box<dyn ValueConverter> {
        match dst {
            Float16 => Box::new(Fix16ToFloat16Converter { bit: src_bit }),
            Float => Box::new(Fix16ToFloatConverter { bit: src_bit }),
            _ => Box::new(SelfConverter {
                value_type: Fix16 as i32,
            }),
        }
    }

    fn check(dst: &ValueType) -> bool {
        [Float16, Float].contains(dst)
    }
}

pub struct Complex16ConverterFactory;

impl ValueConverterFactory for Complex16ConverterFactory {
    fn create(dst: &ValueType, src_bit: u32, dst_bit: u32) -> Box<dyn ValueConverter> {
        match dst {
            Complex => Box::new(Complex16ToComplexConverter),
            Float32 => Box::new(ComplexToFloat32Converter),
            _ => Box::new(SelfConverter { value_type: Complex16 as i32 })
        }
    }

    fn check(dst: &ValueType) -> bool {
        [Complex].contains(dst)
    }
}

pub struct ComplexConverterFactory;

impl ValueConverterFactory for ComplexConverterFactory {
    fn create(dst: &ValueType, src_bit: u32, dst_bit: u32) -> Box<dyn ValueConverter> {
        match dst {
            Complex16 => Box::new(ComplexToComplex16Converter),
            Float32 => Box::new(ComplexToFloat32Converter),
            _ => Box::new(SelfConverter { value_type: Complex as i32 })
        }
    }

    fn check(dst: &ValueType) -> bool {
        [Complex16].contains(dst)
    }
}

pub struct ConverterFactory;

impl ConverterFactory {
    pub(crate) fn create(
        src: &ValueType,
        dst: &ValueType,
        src_bit: u32,
        dst_bit: u32,
    ) -> Box<dyn ValueConverter> {
        match src {
            Float32 => Float32ConverterFactory::create(dst, src_bit, dst_bit),
            Float16 => Float16ConverterFactory::create(dst, src_bit, dst_bit),
            Float => FloatConverterFactory::create(dst, src_bit, dst_bit),
            Complex16 => Complex16ConverterFactory::create(dst, src_bit, dst_bit),
            Complex => ComplexConverterFactory::create(dst, src_bit, dst_bit),
            Fix32 => Fix32ConverterFactory::create(dst, src_bit, dst_bit),
            Fix16 => Fix16ConverterFactory::create(dst, src_bit, dst_bit),
            ValueTypeNum => Box::new(SelfConverter { value_type: 0 }),
        }
    }

    pub fn check(src: &ValueType, dst: &ValueType) -> bool {
        match src {
            Float32 => Float32ConverterFactory::check(dst),
            Float16 => Float16ConverterFactory::check(dst),
            Float => FloatConverterFactory::check(dst),
            Complex16 => Complex16ConverterFactory::check(dst),
            Complex => ComplexConverterFactory::check(dst),
            Fix32 => Fix32ConverterFactory::check(dst),
            Fix16 => Fix16ConverterFactory::check(dst),
            ValueTypeNum => false,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ValueType {
    Float32 = 0,
    Float16,
    Float,
    Complex16,
    Complex,
    Fix32,
    Fix16,
    ValueTypeNum,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Float32 => write!(f, "{}", "float32"),
            Float16 => write!(f, "{}", "float16"),
            Float => write!(f, "{}", "float"),
            Complex16 => write!(f, "{}", "complex16"),
            Complex => write!(f, "{}", "complex"),
            Fix32 => write!(f, "{}", "fix32"),
            Fix16 => write!(f, "{}", "fix16"),
            ValueTypeNum => write!(f, "{}", "value_type_num"),
        }
    }
}

impl ValueType {
    pub fn get_value_type(string: &str) -> ValueType {
        match string {
            "float32" => Float32,
            "float16" => Float16,
            "float" => Float,
            "fix32" => Fix32,
            "fix16" => Fix16,
            "complex16" => Complex16,
            "complex" => Complex,
            &_ => {
                println!("Invalid value type: {}", string);
                ValueTypeNum
            }
        }
    }
}
