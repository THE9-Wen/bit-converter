use std::fs::File;
use std::io::{BufRead, BufReader, Error, stdin, Write};
use std::path::Path;
use std::str::FromStr;
use eframe::{egui, Frame};
use eframe::egui::{Context};

use regex::Regex;

use crate::common_converter::ValueConverter;
use crate::float_converter::FloatToFloat32Converter;
use crate::value_converter_factory::{ConverterFactory, ValueConverterFactory, ValueType};
use crate::value_converter_factory::ValueType::{Complex, Complex16, Fix16, Fix32, Float, Float16, Float32};

mod value_converter_factory;
mod float_converter;
mod float32_converter;
mod float16_converter;
mod fix32_converter;
mod fix16_converter;
mod complex_converter;
mod complex16_converter;
mod fix_complex16_converter;
mod common_converter;

fn main() -> Result<(), eframe::Error> {
    // 创建视口选项，设置视口的内部大小为320x240像素
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // 运行egui应用程序
    eframe::run_native(
        "Bit Converter", // 应用程序的标题
        options, // 视口选项
        Box::new(|cc| {
            // 为我们提供图像支持
            egui_extras::install_image_loaders(&cc.egui_ctx);
            // 创建并返回一个实现了eframe::App trait的对象
            Ok(Box::new(BitConverter::new(cc)))
        }),
    )
}

struct BitConverter {
    converter: Box<dyn ValueConverter>,
    src_file: String,
    dst_file: String,
    src_value: String,
    dst_value: String,
    src: ValueType,
    dst: ValueType,
}

impl BitConverter {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            converter: Box::new(FloatToFloat32Converter),
            src_file: "".to_string(),
            dst_file: "".to_string(),
            src_value: "".to_string(),
            dst_value: "".to_string(),
            src: Float,
            dst: Float32,
        }
    }

    fn switch_converter(&mut self) -> bool {
        let converter = ConverterFactory::create(&self.src, &self.dst);
        if converter.is_self_converter() {
            true
        } else {
            self.converter = converter;
            println!("Convert form {} to {}.", self.src, self.dst);
            false
        }
    }
}

impl eframe::App for BitConverter {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut switch_converter = false;
            let src = Float;
            let dst = Float32;

            ui.horizontal(|ui| {
                let label = ui.label("Input Data Type:");
                egui::ComboBox::new("input_data_type", "")
                    .selected_text(format!("{}", self.src))
                    .show_ui(ui, |ui| {
                        for kind in [
                            Float, Float32, Float16, Fix32, Fix16, Complex, Complex16
                        ] {
                            switch_converter |= ui.selectable_value(&mut self.src, kind, format!("{}", kind)).changed();
                        }
                    });
            });

            ui.horizontal(|ui| {
                let label = ui.label("Output Data Type:");
                egui::ComboBox::new("output_data_type", "")
                    .selected_text(format!("{}", self.dst))
                    .show_ui(ui, |ui| {
                        for kind in [
                            Float, Float32, Float16, Fix32, Fix16, Complex, Complex16
                        ] {
                            switch_converter |= ui.selectable_value(&mut self.dst, kind, format!("{}", kind)).changed();
                        }
                    });
            });
            if switch_converter && self.switch_converter() {
                self.src = src;
                self.dst = dst;
            }

            ui.label("Convert File:");
            ui.horizontal(|ui| {
                let label = ui.label("Input File Path:");
                ui.text_edit_singleline(&mut self.src_file);
                if ui.button("Browse").clicked() {
                    println!("Hello");
                }
            });
            ui.horizontal(|ui| {
                let label = ui.label("Output File Path:");
                ui.text_edit_singleline(&mut self.dst_file);
            });
            if ui.button("Convert").clicked() {
                process_file(self.src_file.as_ref(), self.converter.as_ref());
            }

            ui.label("Convert Value:");
            ui.horizontal(|ui| {
                let label = ui.label("Input Value:");
                ui.text_edit_singleline(&mut self.src_value);
            });
            ui.horizontal(|ui| {
                let label = ui.label("Output Value:");
                ui.text_edit_singleline(&mut self.dst_value);
            });
            if ui.button("Convert").clicked() {
                self.dst_value = self.converter.convert(self.src_value.as_ref());
            }
        });
    }
}

// fn main() -> Result<(), Error> {
//     println!("***********Value Converter**********
// Supported value type:
// 1. float32
// 2. float16
// 3. float
// 4. fix32
// 5. fix16
// 6. complex16
// 7. complex
// Current converter: float32 to float
// Input a number or file path to convert
// Input two value types divided by a blank space to switch converter
// Input \"quit\" to quit
// ************************************");
//     let mut input = String::new();
//     let mut trimmed_input;
//     let buffer = stdin();
//     let mut converter = ConverterFactory::create(Float, Float32);
//     let bit_stream = Regex::new(r"^(0x)[0-9A-Fa-f]{1,8}$").unwrap();
//     let number = Regex::new(r"^[+-]?(\d+(\.\d*)?|\.\d+)$").unwrap();
//     loop {
//         input.clear();
//         buffer.read_line(&mut input).unwrap();
//         trimmed_input = input.trim();
//         let split: Vec<&str> = trimmed_input.split(" ").collect();
//         if split.len() == 2 {
//             converter = ConverterFactory::create(
//                 ValueType::get_value_type(split[0]),
//                 ValueType::get_value_type(split[1]));
//         } else if trimmed_input == "quit" {
//             return Ok(());
//         } else if bit_stream.is_match(trimmed_input) {
//             println!("{}", converter.convert(&trimmed_input[2..]));
//         } else if number.is_match(trimmed_input) {
//             println!("{}", converter.convert(trimmed_input));
//         } else {
//             process_file(&trimmed_input, &*converter);
//         }
//     }
// }

fn process_file(path: &str, converter: &dyn ValueConverter) {
    let path = Path::new(path);
    let file = File::open(path);
    match file {
        Ok(file) => {
            let buffered = BufReader::new(file);
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_dir = path.parent().unwrap_or(".".as_ref());
            let file_extension = path.extension().unwrap_or("".as_ref()).to_str().unwrap();
            let mut file_out = File::create(Path::join(file_dir, format!("{}_out.{}", &file_name[..(file_name.len() - file_extension.len() - 1)], file_extension))).unwrap();
            for line in buffered.lines() {
                match line {
                    Ok(line) => {
                        let value;
                        if line.starts_with("0x") {
                            value = converter.convert(&line[2..]);
                        } else {
                            value = converter.convert(&line);
                        }
                        file_out.write_all(format!("{}\n", value).as_bytes()).unwrap();
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
