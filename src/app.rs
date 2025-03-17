use eframe::egui::{self, Align, DroppedFile, Image, SidePanel, Slider, TopBottomPanel};
use image::ImageReader;

use crate::{SAMPLE_RATE, common::SSTVMode, martinm1::MartinM1};

pub struct RSSTV {
    image_path: Option<String>,
    volume: usize,
}

impl Default for RSSTV {
    fn default() -> Self {
        RSSTV {
            image_path: None,
            volume: 50,
        }
    }
}

impl eframe::App for RSSTV {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        SidePanel::left("encoder").show(ctx, |ui| {
            ui.label("SSTV Transcoder").highlight();
            ui.add(Slider::new(&mut self.volume, 0..=100));
            if ui.button("Encode").clicked() {
                let mut encoder = MartinM1::new();
                let opened = ImageReader::open(&self.image_path.clone().unwrap()).unwrap();
                let samples = encoder.encode(opened.decode().unwrap());
                wavers::write("out.wav", &samples.to_samples(), SAMPLE_RATE as i32, 1).unwrap();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            preview_files_being_dropped(ctx);

            if let Some(path) = &self.image_path {
                ui.image(format!("file://{}", path));
            } else {
                ui.label("Drag and drop an image!");
            }
        });

        ctx.input_mut(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.image_path = Some(
                    i.raw.dropped_files[i.raw.dropped_files.len() - 1]
                        .clone()
                        .path
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                );
            }
            i.raw.dropped_files = Vec::new();
        })
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::{Align2, Color32, Id, LayerId, Order, TextStyle};
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i: &egui::InputState| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
