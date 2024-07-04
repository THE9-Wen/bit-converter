use std::fmt::{Display, Formatter};
use std::io::stdin;
use std::str::FromStr;

use crate::common_converter::{SelfConverter, ValueConverter};
use crate::fix16_converter::{Fix16ToFloat16Converter, Fix16ToFloatConverter};
use crate::float16_converter::{Float16ToFix16Converter, Float16ToFix32Converter, Float16ToFloat32Converter, Float16ToFloatConverter};
use crate::float32_converter::{Float32ToComplexConverter, Float32ToFix16Converter, Float32ToFix32Converter, Float32ToFloat16Converter, Float32ToFloatConverter};
use crate::float_converter::{FloatToFix16Converter, FloatToFix32Converter, FloatToFloat16Converter, FloatToFloat32Converter};
use crate::value_converter_factory::ValueType::{Complex, Complex16, Fix16, Fix32, Float, Float16, Float32, ValueTypeNum};

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

pub trait ValueConverterFactory {
    fn create(dst: &ValueType) -> Box<dyn ValueConverter>;
}

pub struct FloatConverterFactory;

impl ValueConverterFactory for FloatConverterFactory {
    fn create(dst: &ValueType) -> Box<dyn ValueConverter> {
        match dst {
            Float32 => Box::new(FloatToFloat32Converter),
            Float16 => Box::new(FloatToFloat16Converter),
            Complex16 => Box::new(FloatToFix32Converter { bit: 0 }),
            Complex => Box::new(FloatToFix16Converter { bit: 0 }),
            Fix32 => create_fix_converter(|bit| Box::new(FloatToFix32Converter { bit })),
            Fix16 => create_fix_converter(|bit| Box::new(FloatToFix16Converter { bit })),
            _ => Box::new(SelfConverter { value_type: Float as i32 }),
        }
    }
}

pub struct Float32ConverterFactory;

impl ValueConverterFactory for Float32ConverterFactory {
    fn create(dst: &ValueType) -> Box<dyn ValueConverter> {
        match dst {
            Float16 => Box::new(Float32ToFloat16Converter),
            Float => Box::new(Float32ToFloatConverter),
            Complex => Box::new(Float32ToComplexConverter),
            Fix32 => create_fix_converter(|bit| Box::new(Float32ToFix32Converter { bit })),
            Fix16 => create_fix_converter(|bit| Box::new(Float32ToFix16Converter { bit })),
            _ => Box::new(SelfConverter { value_type: Float32 as i32 })
        }
    }
}

pub struct Float16ConverterFactory;

impl ValueConverterFactory for Float16ConverterFactory {
    fn create(dst: &ValueType) -> Box<dyn ValueConverter> {
        match dst {
            Float32 => Box::new(Float16ToFloat32Converter),
            Float => Box::new(Float16ToFloatConverter),
            Fix32 => create_fix_converter(|bit| Box::new(Float16ToFix32Converter { bit })),
            Fix16 => create_fix_converter(|bit| Box::new(Float16ToFix16Converter { bit })),
            _ => Box::new(SelfConverter { value_type: Float16 as i32 })
        }
    }
}

pub struct Fix32ConverterFactory;

impl ValueConverterFactory for Fix32ConverterFactory {
    fn create(dst: &ValueType) -> Box<dyn ValueConverter> {
        match dst {
            // Float32 => {}
            // Float16 => {}
            // Float => Box::new()
            // Complex16 => {}
            // Complex => {}
            // Fix16 => {}
            _ => Box::new(SelfConverter { value_type: Fix32 as i32 })
        }
    }
}

pub struct Fix16ConverterFactory;

impl ValueConverterFactory for Fix16ConverterFactory {
    fn create(dst: &ValueType) -> Box<dyn ValueConverter> {
        match dst {
            Float16 => create_fix_converter(|bit| Box::new(Fix16ToFloat16Converter { bit })),
            Float => create_fix_converter(|bit| Box::new(Fix16ToFloatConverter { bit })),
            _ => Box::new(SelfConverter { value_type: Fix16 as i32 }),
        }
    }
}

pub struct Complex16ConverterFactory;

impl ValueConverterFactory for Complex16ConverterFactory {
    fn create(dst: &ValueType) -> Box<dyn ValueConverter> {
        todo!()
    }
}

pub struct ComplexConverterFactory;

impl ValueConverterFactory for ComplexConverterFactory {
    fn create(dst: &ValueType) -> Box<dyn ValueConverter> {
        todo!()
    }
}

pub struct ConverterFactory;

impl ConverterFactory {
    pub(crate) fn create(src: &ValueType, dst: &ValueType) -> Box<dyn ValueConverter> {
        match src {
            Float32 => Float32ConverterFactory::create(dst),
            Float16 => Float16ConverterFactory::create(dst),
            Float => FloatConverterFactory::create(dst),
            Complex16 => Complex16ConverterFactory::create(dst),
            Complex => ComplexConverterFactory::create(dst),
            Fix32 => Fix32ConverterFactory::create(dst),
            Fix16 => Fix16ConverterFactory::create(dst),
            ValueTypeNum => Box::new(SelfConverter { value_type : 0 }),
        }
    }
}

fn create_fix_converter<F>(f: F) -> Box<dyn ValueConverter> where
    F: FnOnce(u32) -> Box<dyn ValueConverter>
{
    let buffer = stdin();
    let mut input = String::new();
    let converter;
    loop {
        println!("Please input dst fraction bit width:");
        buffer.read_line(&mut input).unwrap();
        let bit = u32::from_str(&input.trim());
        match bit {
            Ok(bit) => {
                converter = f(bit);
                break;
            }
            Err(_) => {}
        }
        println!("Invalid input!");
    }
    converter
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
            "complex"=> Complex,
            &_ => {
                println!("Invalid value type: {}", string);
                ValueTypeNum
            },
        }
    }
}