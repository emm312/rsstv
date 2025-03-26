use std::f64::consts::PI;

use image::DynamicImage;
use wavers::Samples;

use crate::SAMPLE_RATE;

/// A frequency component struct, consists of a frequency and duration.
/// A SSTV signal is made up of a single-tone, frequency modulated to encode the image,
/// therefore meaning this is the most basic unit a SSTV signal can be split up to.
pub struct Component {
    pub freq: usize,
    pub len_us: f64,
}

/// A whole SSTV signal is just a list of freq components.
pub struct Signal {
    inner: Vec<Component>,
}

impl Signal {
    pub fn new() -> Signal {
        Signal { inner: Vec::new() }
    }

    /// Get the time-domain samples as opposed to our hybrid
    /// time and frequency domain data.
    ///
    /// This is what gets written to the WAV file.
    ///
    /// TODO: Consider switching to outputting a f32 rather than i16
    pub fn to_samples(&self) -> Samples<i16> {
        let mut samples = Vec::new();
        let mut phase: f64 = 0.;

        for component in self.inner.iter() {
            let total_length = ((component.len_us as f64 / 1000000.) * SAMPLE_RATE as f64) as usize;
            for _ in 0..total_length {
                samples.push((phase.sin() * 10000.) as i16);
                phase += 2. * PI * component.freq as f64 / SAMPLE_RATE as f64;
            }
        }

        Samples::from(samples)
    }

    /// Add a new frequency component to the signal.
    pub fn push(&mut self, freq: usize, len_us: f64) {
        self.inner.push(Component { freq, len_us });
    }
}

/// The SSTVMode trait. This trait encompasses all the functions required to implement
/// a new mode, consisting of 4 functions, primarily `encode` and `decode`.
///
/// TODO: Move general things from the Martin M1 encoder out and implement more modes.
pub trait SSTVMode {
    fn new() -> Self;
    fn encode(&mut self, image: DynamicImage) -> Signal;
    fn decode(&mut self, _audio: &[f32]) -> DecodeResult {
        todo!()
    }
    fn get_image(&self) -> DynamicImage;
}

/// Check if two floats are within 250 of eachother.
/// Used for decoding.
pub fn within_250hz(a: f64, b: f64) -> bool {
    (a - b).abs() < 250.
}

/// This struct contains the output of the DSP chain - A slice of f64's and
/// a position through the slice.
///
/// This struct is used when decoding signals, providing an easy way to iterate
/// over the elements.
///
/// TODO: add `take_while_freq_for_atleast` to make header & seperator tone
/// detection more rigorous
pub struct DSPOut<'a> {
    pub inner: &'a [f64],
    pos: usize,
}

impl<'a> DSPOut<'a> {
    pub fn new(from: &[f64]) -> DSPOut {
        DSPOut {
            inner: from,
            pos: 0,
        }
    }

    /// This function will consume samples until they deviate by more than 250hz from
    /// the `frq` argument, returning Some(()) if it was successful.
    pub fn take_while_frq(&mut self, frq: f64) -> Option<()> {
        let mut at = 0;
        for i in self.pos..self.inner.len() {
            if !within_250hz(*self.inner.get(i)?, frq) {
                at = i;
                break;
            }
        }

        self.pos = at;
        Some(())
    }

    /// This function will consume samples until they are less than 250Hz from
    /// the `frq` argument, returning Some(()) if it was successful.
    pub fn take_till_frq(&mut self, frq: f64) -> Option<()> {
        let mut at = 0;
        for i in self.pos..self.inner.len() {
            if within_250hz(*self.inner.get(i)?, frq) {
                at = i;
                break;
            }
        }

        self.pos = at;
        Some(())
    }

    /// Consume `us` micro-seconds worth of samples, returning Some with
    /// the average frequency throught said samples if successful.
    pub fn take_us(&mut self, us: f64) -> Option<f64> {
        let total_samples = us_to_n_samples(us);

        let mut sum = 0.;

        for i in self.pos..(self.pos + total_samples) {
            sum += self.inner.get(i)?;
        }

        self.pos += total_samples;

        Some(sum / (total_samples as f64))
    }

    /// Set the position over the samples
    pub fn set_to(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// Get the position over the samples
    pub fn get_pos(&self) -> usize {
        self.pos
    }
}

pub fn us_to_n_samples(s: f64) -> usize {
    (SAMPLE_RATE as f64 * (s / 1_000_000.)).round() as usize
}

fn n_samples_to_us(samples: usize) -> f64 {
    (samples as f64 / SAMPLE_RATE as f64) * 1_000_000.
}

/// A decode result. Either finished, partial, or no image was found.
pub enum DecodeResult {
    Finished(DynamicImage),
    Partial(DynamicImage),
    NoneFound,
}
