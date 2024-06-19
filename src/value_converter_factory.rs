use std::io::stdin;
use std::str::FromStr;

use crate::common_converter::{SelfConverter, ValueConverter};
use crate::complex16_converter::Complex16ToComplexConverter;
use crate::complex_converter::{ComplexToComplex16Converter, ComplexToFloat16Converter, ComplexToFloat32Converter, ComplexToFloatConverter};
use crate::fix16_converter::{Fix16ToFix16Converter, Fix16ToFloat16Converter, Fix16ToFloat32Converter, Fix16ToFloatConverter};
use crate::fix32_converter::{Fix32ToComplex16Converter, Fix32ToComplexConverter, Fix32ToFloat16Converter, Fix32ToFloat32Converter, Fix32ToFloatConverter};
use crate::float16_converter::{Float16ToFix16Converter, Float16ToFix32Converter, Float16ToFloat32Converter, Float16ToFloatConverter};
use crate::float32_converter::{Float32ToComplexConverter, Float32ToFix16Converter, Float32ToFix32Converter, Float32ToFloat16Converter, Float32ToFloatConverter};
use crate::float_converter::{FloatToComplex16Converter, FloatToComplexConverter, FloatToFix16Converter, FloatToFix32Converter, FloatToFloat16Converter, FloatToFloat32Converter};
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
    fn create(dst: ValueType) -> Box<dyn ValueConverter>;
}

pub struct FloatConverterFactory;

impl ValueConverterFactory for FloatConverterFactory {
    fn create(dst: ValueType) -> Box<dyn ValueConverter> {
        match dst {
            Float32 => Box::new(FloatToFloat32Converter),
            Float16 => Box::new(FloatToFloat16Converter),
            Complex16 => Box::new(FloatToFix32Converter { bit: 0 }),
            Complex => Box::new(FloatToFix16Converter { bit: 0 }),
            Fix32 => create_fix_converter(|bit| FloatToFix32Converter { bit }),
            Fix16 => create_fix_converter(|bit| FloatToFix16Converter { bit }),
            _ => Box::new(SelfConverter { value_type: Float as i32 }),
        }
    }
}

pub struct Float32ConverterFactory;

impl ValueConverterFactory for Float32ConverterFactory {
    fn create(dst: ValueType) -> Box<dyn ValueConverter> {
        match dst {
            Float16 => Box::new(Float32ToFloat16Converter),
            Float => Box::new(Float32ToFloatConverter),
            Complex => Box::new(Float32ToComplexConverter),
            Fix32 => create_fix_converter(|bit| Float32ToFix32Converter { bit }),
            Fix16 => create_fix_converter(|bit| Float32ToFix16Converter { bit }),
            _ => Box::new(SelfConverter { value_type: Float32 as i32 })
        }
    }
}

pub struct Float16ConverterFactory;

impl ValueConverterFactory for Float16ConverterFactory {
    fn create(dst: ValueType) -> Box<dyn ValueConverter> {
        match dst {
            Float32 => Box::new(Float16ToFloat32Converter),
            Float => Box::new(Float16ToFloatConverter),
            Fix32 => create_fix_converter(|bit|Float16ToFix32Converter { bit }),
            Fix16 => create_fix_converter(|bit|Float16ToFix16Converter { bit }),
            _ => Box::new(SelfConverter { value_type: Float16 as i32 })
        }
    }
}

pub struct Fix32ConverterFactory;

impl ValueConverterFactory for Fix32ConverterFactory {
    fn create(dst: ValueType) -> Box<dyn ValueConverter> {
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

fn create_fix_converter<F>(f: F) -> Box<dyn ValueConverter> where
    F: FnOnce() -> dyn ValueConverter
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
                converter = Box::new(f(bit));
                break;
            }
            Err(_) => {}
        }
        println!("Invalid input!");
    }
    converter
}

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

impl ValueType {
    pub fn get_value_type(string: &str) -> usize {
        let value = match string {
            "float32" => Float32,
            "float16" => Float16,
            "float" => Float,
            "fix32" => Fix32,
            "fix16" => Fix16,
            "complex" => Complex16,
            &_ => ValueTypeNum,
        };
        value as usize
    }
}