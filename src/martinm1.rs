use std::u8;

use image::{ColorType, DynamicImage, GenericImageView, Pixel, imageops::FilterType};
use wavers::write;

use crate::{common::{SSTVMode, Signal}, dsp, SAMPLE_RATE};

pub struct MartinM1 {
    decoded_image: DynamicImage,
    samples: Vec<i16>,
}

impl SSTVMode for MartinM1 {
    fn new() -> Self {
        MartinM1 {
            decoded_image: DynamicImage::new(320, 256, ColorType::Rgb16),
            samples: Vec::new(),
        }
    }

    fn encode(&mut self, image: image::DynamicImage) -> Signal {
        let resize = image.resize_exact(320, 256, FilterType::Nearest);
        let mut out = Signal::new();

        out.push(1900, 300_000.);
        out.push(1200, 10_000.);
        out.push(1900, 300_000.);

        //0b1011001 VIS code

        // start bit
        out.push(1200, 30_000.);

        out.push(1100, 30_000.);
        out.push(1300, 30_000.);
        out.push(1300, 30_000.);
        out.push(1100, 30_000.);
        out.push(1100, 30_000.);
        out.push(1300, 30_000.);
        out.push(1100, 30_000.);

        // stop bit
        out.push(1200, 30_000.);

        for i in 0..256 {
            sync(&mut out);
            colour_sep(&mut out);

            for colour in [1, 2, 0] {
                for j in 0..320 {
                    let pixel = resize.get_pixel(j, i);
                    let rgb = pixel.to_rgb();
                    let channels = rgb.channels();
                    let value = channels[colour] as f64 / u8::MAX as f64;

                    // modulating frequency is a range between 2300 and 1500Hz
                    let range = 2300. - 1500.;
                    let freq = value * range;
                    out.push(freq as usize + 1500, 457.6);
                }
                colour_sep(&mut out);
            }
        }

        out
    }

    fn decode(&mut self, audio: &Vec<i16>) {
        self.samples.append(&mut audio.clone());

        let out: &[i16] = &dsp::quadrature_demod(&audio).iter().map(|n| *n as i16).collect::<Vec<i16>>();
        write("dsp_out.wav", out, SAMPLE_RATE as i32, 1).unwrap();
    }

    fn get_image(&self) -> image::DynamicImage {
        self.decoded_image.clone()
    }
}

fn sync(out: &mut Signal) {
    out.push(1200, 4862.);
}

fn colour_sep(out: &mut Signal) {
    out.push(1500, 572.);
}
