use std::u8;

use image::{imageops::FilterType, ColorType, DynamicImage, GenericImage, GenericImageView, Pixel, RgbImage};
use simple_plot::plot;

use crate::{
    SAMPLE_RATE,
    common::{self, DSPOut, SSTVMode, Signal, within_50hz},
    dsp,
};

pub struct MartinM1 {
    decoded_image: DynamicImage,
    samples: Vec<i16>,
    current_pos: usize,
}

impl SSTVMode for MartinM1 {
    fn new() -> Self {
        MartinM1 {
            decoded_image: DynamicImage::new(320, 256, ColorType::Rgb16),
            samples: Vec::new(),
            current_pos: 0,
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
        out.push(0, 1000_000.);
        out
    }

    fn decode(&mut self, audio: &Vec<i16>) -> DynamicImage {
        self.samples.append(&mut audio.clone());

        let demod_waveform = dsp::quadrature_demod(&audio);

        let mut out = DSPOut::new(&demod_waveform);

        self.get_calibration_header(&mut out);

        let mut image = DynamicImage::new(320, 256, ColorType::Rgb16);

        for i in 0..256 {
            out.take_till_frq(1200.);
            out.take_while_frq(1200.);

            out.take_till_frq(1500.);
            out.take_while_frq(1500.);
            for colour in [1, 2, 0] {
                for j in 0..320 {
                    let val = out.take_us(457.6).unwrap();

                    let brightness = (val - 1500.) / (2300. - 1500.);

                    let mut rgb = image.get_pixel(j, i);

                    rgb.channels_mut()[colour] = (brightness * 255.) as u8;

                    image.put_pixel(j, i, rgb);
                }
                out.take_till_frq(1500.);
                out.take_while_frq(1500.);
            }
        }
        image
    }

    fn get_image(&self) -> image::DynamicImage {
        self.decoded_image.clone()
    }
}

impl MartinM1 {
    fn get_calibration_header(&mut self, sig: &mut DSPOut) -> Option<u8> {
        sig.take_till_frq(1900.)?;

        sig.take_while_frq(1900.)?;

        sig.take_till_frq(1200.)?;

        sig.take_till_frq(1900.)?;
        sig.take_while_frq(1900.)?;

        let vis = [
            sig.take_us(30_000.)? < 1200.,
            sig.take_us(30_000.)? < 1200.,
            sig.take_us(30_000.)? < 1200.,
            sig.take_us(30_000.)? < 1200.,
            sig.take_us(30_000.)? < 1200.,
            sig.take_us(30_000.)? < 1200.,
        ];

        let parity = sig.take_us(30_000.)? < 1200.;

        // even parity check
        //assert!(
        //    vis.iter().map(|b| *b as u8).sum::<u8>() % 2 != parity as u8,
        //    "parity bit failed"
        //);

        sig.take_while_frq(1200.)?;

        let mut total = 0;

        for (pos, elem) in vis.iter().enumerate() {
            total += 2_u8.pow((6 - pos) as u32) * (*elem as u8);
        }

        Some(total)
    }
}

fn sync(out: &mut Signal) {
    out.push(1200, 4862.);
}

fn colour_sep(out: &mut Signal) {
    out.push(1500, 572.);
}
