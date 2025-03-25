use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type};
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, Pixel, imageops::FilterType};

use crate::{
    SAMPLE_RATE,
    common::{DSPOut, DecodeResult, SSTVMode, Signal},
    dsp,
};

/// A struct implementing the Martin M1 SSTV mode
///
/// eg:
/// ```rs
/// let mut mode = MartinM1::new();
///
/// let mut image = ImageReader::open("file.png").unwrap();
/// let samples = vec![...];
///
/// let encoded_audio = mode.encode(image);
///
/// let decoded_image = mode.decode(samples);
/// ```
pub struct MartinM1 {
    /// A cache of the decoded image to speed up decodes
    decoded_image: DynamicImage,
    /// Buffer of every sample accumulated - calling MartinM1::decode adds the passed sample list
    /// to this vec
    samples: Vec<f32>,

    // Used for caching in live decodes
    in_partial_decode: bool,
    pos: usize,
    row: u32,
}

// Documentation for `MartinM1::encode` and `MartinM1::decode` can be found in the SSTVMode trait

impl SSTVMode for MartinM1 {
    fn new() -> Self {
        MartinM1 {
            decoded_image: DynamicImage::new(320, 256, ColorType::Rgb16),
            samples: Vec::new(),
            in_partial_decode: false,
            row: 0,
            pos: 0,
        }
    }

    fn encode(&mut self, image: image::DynamicImage) -> Signal {
        let resize = image.resize_exact(320, 256, FilterType::Nearest);
        let mut out = Signal::new();

        // Start header
        // Comprised of a 1900Hz 300ms leader tone, followed by a 1200Hz 10ms break and another leader
        out.push(1900, 300_000.);
        out.push(1200, 10_000.);
        out.push(1900, 300_000.);

        // 0b1011001 VIS code
        // 30ms 1200Hz leader followed by 7 30ms long bits - 1100Hz for a 1 and 1300Hz for a 0

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

        // Loop through rows in the image
        for i in 0..256 {
            // Sync & colour sep
            sync(&mut out);
            colour_sep(&mut out);

            // Do colour channels in order BGR rather than RGB
            for colour in [1, 2, 0] {
                // Go through the scanline
                for j in 0..320 {
                    // Grab the pixels value corresponding to the correct colour channel
                    let pixel = resize.get_pixel(j, i);
                    let rgb = pixel.to_rgb();
                    let channels = rgb.channels();
                    // Calculate a % luminance for said colour, this being multiplied with the total modulating
                    // range to figure out frequency
                    let value = channels[colour] as f64 / u8::MAX as f64;

                    // Calculating modulating frequency in a range between 2300 and 1500Hz, each colour being 457.6μs long
                    let range = 2300. - 1500.;
                    let freq = value * range;
                    out.push(freq as usize + 1500, 457.6);
                }
                colour_sep(&mut out);
            }
        }
        // Add a 100ms break at the end
        out.push(0, 100_000.);
        out
    }

    fn decode(&mut self, audio: &[f32]) -> DecodeResult {
        // Accumulate next chunk of samples into internal buffer
        self.samples.append(&mut audio.to_vec());

        // IIR Bandpass filter, 1KHz to 3KHz passband
        // TODO: some form of caching to speedup live decodes
        // as self.samples grows from the stream from the microphone
        // the filter will have to recalculate across every sample every time a
        // live decode is requested, hindering lower buffer sizes

        let fl = 1.khz();
        let fh = 3.khz();
        let fs = SAMPLE_RATE.hz();

        let coeffs_lp = Coefficients::<f64>::from_params(Type::LowPass, fs, fh, 1.).unwrap();
        let coeffs_hp = Coefficients::<f64>::from_params(Type::HighPass, fs, fl, 1.).unwrap();

        let mut biquad_lp = DirectForm1::<f64>::new(coeffs_lp);
        let mut biquad_hp = DirectForm1::<f64>::new(coeffs_hp);

        // TODO: Have less allocations here. (Vec::with_capacity will take ages across the millions of samples
        // it grows to)
        // Would likely speed it up tenfold

        let mut filtered_lp = Vec::with_capacity(audio.len());

        for elem in &self.samples {
            filtered_lp.push(biquad_lp.run(*elem as f64));
        }

        let mut filtered_hp = Vec::with_capacity(audio.len());

        for elem in filtered_lp {
            filtered_hp.push(biquad_hp.run(elem));
        }

        // Perform a quadrature demodulation on the filtered signal
        let res = dsp::quadrature_demod(&filtered_hp);

        let mut out = DSPOut::new(&res);

        // Set the position of the cursor over the samples to the spot the last decode ended at
        out.set_to(self.pos);

        // If not in a partial decode, look for the header, exiting if no header is found
        if !self.in_partial_decode {
            if let None = self.get_calibration_header(&mut out) {
                return DecodeResult::NoneFound;
            }
        }

        // Loop through every row, starting from the last decoded row
        for i in self.row..256 {
            // Save the start position of the row for partial decodes so we know where to start
            let start_pos = out.get_pos();

            // If the buffer of samples ends...
            if let None = out
                .take_till_frq(1200.)
                .and_then(|_| out.take_while_frq(1200.))
                .and_then(|_| out.take_us(572.))
            {
                // Save the position over the buffer & retain information about position, returning the image
                self.pos = start_pos;
                self.in_partial_decode = true;
                self.row = i;
                // TODO: remove allocation here
                return DecodeResult::Partial(self.decoded_image.clone());
            }
            // Loop through every colour channel..
            for colour in [1, 2, 0] {
                // And scanline
                for j in 0..320 {
                    // Try take a pixels worth of data, saving if it fails
                    if let Some(val) = out.take_us(457.6) {
                        // Calculate brightness based off of the average freq over the 457.6μs
                        let brightness = (val - 1500.) / (2300. - 1500.);

                        let mut rgb = self.decoded_image.get_pixel(j, i);

                        rgb.channels_mut()[colour] = (brightness * 255.) as u8;
                        // Put colour value back to the image
                        self.decoded_image.put_pixel(j, i, rgb);
                    } else {
                        // If we run out, save data
                        self.pos = start_pos;
                        self.row = i;
                        self.in_partial_decode = true;
                        return DecodeResult::Partial(self.decoded_image.clone());
                    }
                }

                // Try take the colour seperator mark, saving if it fails
                if let None = out.take_us(572.) {
                    self.pos = start_pos;
                    self.row = i;
                    self.in_partial_decode = true;
                    return DecodeResult::Partial(self.decoded_image.clone());
                }
            }
        }

        // If we get through that loop, we successfully decoded the image!
        DecodeResult::Finished(self.decoded_image.clone())
    }

    fn get_image(&self) -> image::DynamicImage {
        self.decoded_image.clone()
    }
}

impl MartinM1 {
    /// This function looks for the calibration header of the samples, returning
    /// the 8 bit VIS code if one is found.
    /// 
    /// TODO: Generalise to all SSTV modes & remove the self argument.
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

        // even parity bit check
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

/// Add a 1200Hz 4.862ms sync tone, this is placed after each scanline finishes
fn sync(out: &mut Signal) {
    out.push(1200, 4862.);
}

/// Add a 1500Hz 572μs seperator tone between consequtive colour channels
fn colour_sep(out: &mut Signal) {
    out.push(1500, 572.);
}
