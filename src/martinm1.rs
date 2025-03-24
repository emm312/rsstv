use std::{time::{Duration, Instant}, u8};

use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type};
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, Pixel, imageops::FilterType};
use num_complex::Complex64;
use simple_plot::plot;

use crate::{
    common::{us_to_n_samples, DSPOut, DecodeResult, SSTVMode, Signal}, dsp, SAMPLE_RATE
};

pub struct MartinM1 {
    decoded_image: DynamicImage,
    samples: Vec<f64>,
    in_partial_decode: bool,
    pos: usize,
    row: u32,
    last_quad_demod: Complex64,
    prev_chunk: Vec<f32>
}

impl SSTVMode for MartinM1 {
    fn new() -> Self {
        MartinM1 {
            decoded_image: DynamicImage::new(320, 256, ColorType::Rgb16),
            samples: Vec::new(),
            in_partial_decode: false,
            row: 0,
            pos: 0,
            last_quad_demod: Complex64::ZERO,
            prev_chunk: Vec::new()
        }
    }

    fn encode(&mut self, image: image::DynamicImage) -> Signal {
        let resize = image.resize_exact(320, 256, FilterType::Nearest);
        let mut out = Signal::new();

        out.push(0, 3_000_000.);

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

    fn decode(&mut self, audio: &Vec<f32>) -> DecodeResult {
        let fl = 1.khz();
        let fh = 3.khz();
        let fs = SAMPLE_RATE.hz();

        let coeffs_lp = Coefficients::<f64>::from_params(Type::LowPass, fs, fh, 1.).unwrap();
        let coeffs_hp = Coefficients::<f64>::from_params(Type::HighPass, fs, fl, 1.).unwrap();

        let mut biquad_lp = DirectForm1::<f64>::new(coeffs_lp);
        let mut biquad_hp = DirectForm1::<f64>::new(coeffs_hp);

        let mut filtered_lp = Vec::with_capacity(audio.len());

        for elem in [self.prev_chunk.clone(), audio.clone()].iter().flatten() {
            filtered_lp.push(biquad_lp.run(*elem as f64));
        }

        let mut filtered_hp = Vec::with_capacity(audio.len());
        for elem in filtered_lp {
            filtered_hp.push(biquad_hp.run(elem));
        }

        let mut res;
        if self.in_partial_decode {
            res = dsp::quadrature_demod(&filtered_hp.split_at(audio.len()).1.to_vec(), self.last_quad_demod)
        } else {
            res = dsp::quadrature_demod(&filtered_hp, self.last_quad_demod)
        }
        self.last_quad_demod = res.1;
        self.samples.append(&mut res.0);

        self.prev_chunk = audio.clone();

        //plot!("a", &demod_waveform);
        let mut out = DSPOut::new(&self.samples);

        out.set_to(self.pos);

        if !self.in_partial_decode {
            if let None = self.get_calibration_header(&mut out) {
                return DecodeResult::NoneFound;
            }
        }

        for i in self.row..256 {
            let start_pos = out.get_pos();

            if let None = out
                .take_till_frq(1200.)
                .and_then(|_| out.take_while_frq(1200.))
                .and_then(|_| out.take_till_frq(1500.))
                .and_then(|_| out.take_while_frq(1500.))
            {
                self.pos = start_pos;
                self.in_partial_decode = true;
                self.row = i;
                return DecodeResult::Partial(self.decoded_image.clone());
            }
            for colour in [1, 2, 0] {
                for j in 0..320 {
                    if let Some(val) = out.take_us(457.6) {
                        let brightness = (val - 1500.) / (2300. - 1500.);

                        let mut rgb = self.decoded_image.get_pixel(j, i);

                        rgb.channels_mut()[colour] = (brightness * 255.) as u8;

                        self.decoded_image.put_pixel(j, i, rgb);
                    } else {
                        self.pos = start_pos;
                        self.row = i;
                        self.in_partial_decode = true;
                        return DecodeResult::Partial(self.decoded_image.clone());
                    }
                }

                if let None = out
                    .take_till_frq(1500.)
                    .and_then(|_| out.take_while_frq(1500.))
                {
                    self.pos = start_pos;
                    self.row = i;
                    self.in_partial_decode = true;
                    return DecodeResult::Partial(self.decoded_image.clone());
                }
            }
        }
        
        DecodeResult::Finished(self.decoded_image.clone())
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

        //println!("header found");

        Some(total)
    }
}

fn sync(out: &mut Signal) {
    out.push(1200, 4862.);
}

fn colour_sep(out: &mut Signal) {
    out.push(1500, 572.);
}
