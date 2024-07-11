use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::str::FromStr;

use eframe::{egui, Frame};
use eframe::egui::{Context, Ui};
use rfd::FileDialog;

use crate::common_converter::ValueConverter;
use crate::float_converter::FloatToFloat32Converter;
use crate::value_converter_factory::{ConverterFactory, ValueConverterFactory, ValueType};
use crate::value_converter_factory::ValueType::{
    Complex, Complex16, Fix16, Fix32, Float, Float16, Float32,
};

mod common_converter;
mod complex16_converter;
mod complex_converter;
mod fix16_converter;
mod fix32_converter;
mod fix_complex16_converter;
mod float16_converter;
mod float32_converter;
mod float_converter;
mod value_converter_factory;

fn main() -> Result<(), eframe::Error> {
    // 创建视口选项，设置视口的内部大小为320x240像素
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };

    // 运行egui应用程序
    eframe::run_native(
        "Bit Converter", // 应用程序的标题
        options,         // 视口选项
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
    src_bit: u32,
    dst_bit: u32,
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
            src_bit: 0,
            dst_bit: 0,
            src: Float,
            dst: Float32,
        }
    }

    fn switch_converter(&mut self) {
        self.converter = ConverterFactory::create(&self.src, &self.dst, self.src_bit, self.dst_bit);
    }

    fn grid_contents(&mut self, ui: &mut Ui) {
        let mut switch_converter = false;
        let src = self.src;
        let dst = self.dst;
        let src_bit = self.src_bit;
        let dst_bit = self.dst_bit;

        ui.label("Input Data Type:");
        ui.horizontal(|ui| {
            egui::ComboBox::new("input_data_type", "")
                .selected_text(format!("{}", self.src))
                .show_ui(ui, |ui| {
                    for kind in [Float, Float32, Float16, Fix32, Fix16, Complex, Complex16] {
                        switch_converter |= ui
                            .selectable_value(&mut self.src, kind, format!("{}", kind))
                            .changed();
                    }
                });
            if self.src == dst {
                self.dst = src;
                self.src_bit = dst_bit;
                self.dst_bit = src_bit;
            }
            match self.src {
                Fix32 => self.select_src_bit(&mut switch_converter, ui,32),
                Fix16 => self.select_src_bit(&mut switch_converter, ui,16),
                _ => {}
            }
        });
        ui.end_row();

        ui.label("Output Data Type:");
        ui.horizontal(|ui| {
            egui::ComboBox::new("output_data_type", "")
                .selected_text(format!("{}", self.dst))
                .show_ui(ui, |ui| {
                    for kind in [Float, Float32, Float16, Fix32, Fix16, Complex, Complex16] {
                        if ConverterFactory::check(&self.src, &kind) {
                            switch_converter |= ui
                                .selectable_value(&mut self.dst, kind, format!("{}", kind))
                                .changed();
                        }
                    }
                });
            match self.dst {
                Fix32 => self.select_src_bit(&mut switch_converter, ui,32),
                Fix16 => self.select_src_bit(&mut switch_converter, ui,16),
                _ => {}
            }
        });
        ui.end_row();

        if switch_converter && ConverterFactory::check(&self.src, &self.dst) {
            self.switch_converter();
        } else {
            self.src = src;
            self.dst = dst;
        }

        ui.label("Convert File:");
        ui.end_row();

        ui.label("Input File Path:");
        ui.text_edit_singleline(&mut self.src_file);
        if ui.button("Browse").clicked() {
            let file = FileDialog::new()
                .add_filter("text", &["txt", "dat"])
                .set_directory("~")
                .pick_file();
            match file {
                None => {}
                Some(file) => {
                    self.src_file = file.clone().to_str().unwrap_or("").to_string();
                    let mut file_name = file.clone().file_name().unwrap().to_os_string().into_string().unwrap_or(String::new());
                    file_name.insert_str(file_name.rfind(".").unwrap_or(0),"_out");
                    self.dst_file = file.clone().parent().unwrap().join(file_name).to_str().unwrap_or("").to_string();
                }
            }
        }
        ui.end_row();

        ui.label("Output File Path:");
        ui.text_edit_singleline(&mut self.dst_file);
        ui.end_row();

        if ui.button("Convert").clicked() {
            process_file(self.src_file.as_ref(), self.converter.as_ref());
        }
        ui.end_row();

        ui.label("Convert Value:");
        ui.end_row();

        ui.label("Input Value:");
        ui.text_edit_multiline(&mut self.src_value);
        ui.end_row();

        ui.label("Output Value:");
        ui.text_edit_multiline(&mut self.dst_value);
        ui.end_row();

        if ui.button("Convert").clicked() {
            let src_values = self.src_value.split("\n");
            self.dst_value = String::new();
            for src_value in src_values {
                if src_value.starts_with("0x") || src_value.starts_with("0X") {
                    self.dst_value.push_str(&format!("{}\n", self.converter.convert(&src_value[2..])));
                } else {
                    self.dst_value.push_str(&format!("{}\n", self.converter.convert(&src_value)));
                }
            }
        }
        ui.end_row();
    }

    fn select_src_bit(&mut self, switch_converter: &mut bool, ui: &mut Ui, range: u32) {
        egui::ComboBox::new("src_bit", "")
            .selected_text(format!("{}", self.src_bit))
            .show_ui(ui, |ui| {
                for kind in 0..range {
                    *switch_converter |= ui
                        .selectable_value(&mut self.src_bit, kind, format!("{}", kind))
                        .changed();
                }
            });
    }

    fn select_dst_bit(&mut self, switch_converter: &mut bool, ui: &mut Ui, range: u32) {
        egui::ComboBox::new("dst_bit", "")
            .selected_text(format!("{}", self.dst_bit))
            .show_ui(ui, |ui| {
                for kind in 0..range {
                    *switch_converter |= ui
                        .selectable_value(&mut self.dst_bit, kind, format!("{}", kind))
                        .changed();
                }
            });
    }
}

impl eframe::App for BitConverter {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("grid")
                .num_columns(2)
                .spacing([20.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.grid_contents(ui);
                });
        });
    }
}

fn process_file(path: &str, converter: &dyn ValueConverter) {
    let path = Path::new(path);
    let file = File::open(path);
    match file {
        Ok(file) => {
            let buffered = BufReader::new(file);
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_dir = path.parent().unwrap_or(".".as_ref());
            let file_extension = path.extension().unwrap_or("".as_ref()).to_str().unwrap();
            let mut file_out = File::create(Path::join(
                file_dir,
                format!(
                    "{}_out.{}",
                    &file_name[..(file_name.len() - file_extension.len() - 1)],
                    file_extension
                ),
            ))
            .unwrap();
            for line in buffered.lines() {
                match line {
                    Ok(line) => {
                        let value;
                        if line.starts_with("0x") {
                            value = converter.convert(&line[2..]);
                        } else {
                            value = converter.convert(&line);
                        }
                        file_out
                            .write_all(format!("{}\n", value).as_bytes())
                            .unwrap();
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
