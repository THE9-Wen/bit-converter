use std::io::stdin;

use crate::common_converter::{SelfConverter, ValueConverter};
use crate::complex16_converter::Complex16ToComplexConverter;
use crate::complex_converter::{ComplexToComplex16Converter, ComplexToFloat16Converter, ComplexToFloat32Converter, ComplexToFloatConverter};
use crate::fix16_converter::{Fix16ToFloat16Converter, Fix16ToFloat32Converter, Fix16ToFloatConverter};
use crate::fix32_converter::{Fix32ToComplex16Converter, Fix32ToComplexConverter, Fix32ToFloat16Converter, Fix32ToFloat32Converter, Fix32ToFloatConverter};
use crate::float16_converter::{Float16ToFix16Converter, Float16ToFix32Converter, Float16ToFloat32Converter, Float16ToFloatConverter};
use crate::float32_converter::{Float32ToComplexConverter, Float32ToFix16Converter, Float32ToFix32Converter, Float32ToFloat16Converter, Float32ToFloatConverter};
use crate::float_converter::{FloatToComplex16Converter, FloatToComplexConverter, FloatToFix16Converter, FloatToFix32Converter, FloatToFloat16Converter, FloatToFloat32Converter};
use crate::value_converter_factory::ValueType::{Complex, Complex16, Fix16, Fix32, Float, Float16, Float32, ValueTypeNum};

pub struct ValueConverterFactory {
    converters: Box<[[Box<dyn ValueConverter>; ValueTypeNum as usize]; 7]>,
}

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


impl ValueConverterFactory {
    pub fn new() -> Self {
        let float32_to_float32: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Float32 as i32 });
        let float32_to_float16: Box<dyn ValueConverter> = Box::new(Float32ToFloat16Converter);
        let float32_to_float: Box<dyn ValueConverter> = Box::new(Float32ToFloatConverter);
        let float32_to_fix32: Box<dyn ValueConverter> = Box::new(Float32ToFix32Converter { bit: 0 });
        let float32_to_fix16: Box<dyn ValueConverter> = Box::new(Float32ToFix16Converter { bit: 0 });
        let float32_to_complex16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Float32 as i32 });
        let float32_to_complex: Box<dyn ValueConverter> = Box::new(Float32ToComplexConverter);

        let float16_to_float32: Box<dyn ValueConverter> = Box::new(Float16ToFloat32Converter);
        let float16_to_float16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Float16 as i32 });
        let float16_to_float: Box<dyn ValueConverter> = Box::new(Float16ToFloatConverter);
        let float16_to_fix32: Box<dyn ValueConverter> = Box::new(Float16ToFix32Converter { bit: 0 });
        let float16_to_fix16: Box<dyn ValueConverter> = Box::new(Float16ToFix16Converter { bit: 0 });
        let float16_to_complex16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Float16 as i32 });
        let float16_to_complex: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Float16 as i32 });

        let float_to_float32: Box<dyn ValueConverter> = Box::new(FloatToFloat32Converter);
        let float_to_float16: Box<dyn ValueConverter> = Box::new(FloatToFloat16Converter);
        let float_to_float: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Float as i32 });
        let float_to_fix32: Box<dyn ValueConverter> = Box::new(FloatToFix32Converter { bit: 0 });
        let float_to_fix16: Box<dyn ValueConverter> = Box::new(FloatToFix16Converter { bit: 0 });
        let float_to_complex16: Box<dyn ValueConverter> = Box::new(FloatToComplex16Converter);
        let float_to_complex: Box<dyn ValueConverter> = Box::new(FloatToComplexConverter);

        let fix32_to_float32: Box<dyn ValueConverter> = Box::new(Fix32ToFloat32Converter { bit: 0 });
        let fix32_to_float16: Box<dyn ValueConverter> = Box::new(Fix32ToFloat16Converter { bit: 0 });
        let fix32_to_float: Box<dyn ValueConverter> = Box::new(Fix32ToFloatConverter { bit: 0 });
        let fix32_to_fix32: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Fix32 as i32 });
        let fix32_to_fix16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Fix32 as i32 });
        let fix32_to_complex16: Box<dyn ValueConverter> = Box::new(Fix32ToComplex16Converter { bit: 0 });
        let fix32_to_complex: Box<dyn ValueConverter> = Box::new(Fix32ToComplexConverter { bit: 0 });

        let fix16_to_float32: Box<dyn ValueConverter> = Box::new(Fix16ToFloat32Converter { bit: 0 });
        let fix16_to_float16: Box<dyn ValueConverter> = Box::new(Fix16ToFloat16Converter { bit: 0 });
        let fix16_to_float: Box<dyn ValueConverter> = Box::new(Fix16ToFloatConverter { bit: 0 });
        let fix16_to_fix32: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Fix16 as i32 });
        let fix16_to_fix16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Fix16 as i32 });
        let fix16_to_complex16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Fix16 as i32 });
        let fix16_to_complex: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Fix16 as i32 });

        let complex_to_float32: Box<dyn ValueConverter> = Box::new(ComplexToFloat32Converter);
        let complex_to_float16: Box<dyn ValueConverter> = Box::new(ComplexToFloat16Converter);
        let complex_to_float: Box<dyn ValueConverter> = Box::new(ComplexToFloatConverter);
        let complex_to_fix32: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex as i32});
        let complex_to_fix16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex as i32});
        let complex_to_complex16: Box<dyn ValueConverter> = Box::new(ComplexToComplex16Converter);
        let complex_to_complex: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex as i32});

        let complex16_to_float32: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex16 as i32 });
        let complex16_to_float16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex16 as i32 });
        let complex16_to_float: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex16 as i32 });
        let complex16_to_fix32: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex16 as i32 });
        let complex16_to_fix16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex16 as i32 });
        let complex16_to_complex16: Box<dyn ValueConverter> = Box::new(SelfConverter { value_type: Complex16 as i32 });
        let complex16_to_complex: Box<dyn ValueConverter> = Box::new(Complex16ToComplexConverter);

        let mut value_converters = Box::new([
            [float32_to_float32, float32_to_float16, float32_to_float, float32_to_fix32, float32_to_fix16, float32_to_complex16, float32_to_complex],
            [float16_to_float32, float16_to_float16, float16_to_float, float16_to_fix32, float16_to_fix16, float16_to_complex16, float16_to_complex],
            [float_to_float32, float_to_float16, float_to_float, float_to_fix32, float_to_fix16, float_to_complex16, float_to_complex],
            [fix32_to_float32, fix32_to_float16, fix32_to_float, fix32_to_fix32, fix32_to_fix16, fix32_to_complex16, fix32_to_complex],
            [fix16_to_float32, fix16_to_float16, fix16_to_float, fix16_to_fix32, fix16_to_fix16, fix16_to_complex16, fix16_to_complex],
            [complex_to_float32, complex_to_float16, complex_to_float, complex_to_fix32, complex_to_fix16, complex_to_complex16, complex_to_complex],
            [complex16_to_float32, complex16_to_float16, complex16_to_float, complex16_to_fix32, complex16_to_fix16, complex16_to_complex16, complex16_to_complex],
        ]);
        Self {
            converters: value_converters,
        }
    }

    pub fn create(&self, src: usize, dst: usize) -> Option<&dyn ValueConverter>{
        if src >= ValueTypeNum as usize || dst >= ValueTypeNum as usize {
            None
        } else {
            if self.converters[src][dst].is_self_converter() {
                println!("self converter");
            }
            Some(self.converters[src][dst].as_ref())
        }
    }
}

pub enum ValueType {
    Float32 = 0,
    Float16,
    Float,
    Fix32,
    Fix16,
    Complex16,
    Complex,
    ValueTypeNum,
}

impl ValueType {
    pub fn get_value_type(string: &str) -> Self {
        match string {
            "float32" => Float32,
            "float16" => Float16,
            "float" => Float,
            "fix32" => Fix32,
            "fix16" => Fix16,
            "complex" => Complex16,
            &_ => ValueTypeNum,
        }
    }
}