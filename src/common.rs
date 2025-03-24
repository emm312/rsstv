use std::f64::consts::PI;

use image::DynamicImage;
use wavers::Samples;

use crate::SAMPLE_RATE;

pub struct Component {
    pub freq: usize,
    pub len_us: f64,
}

pub struct Signal {
    inner: Vec<Component>,
}

impl Signal {
    pub fn new() -> Signal {
        Signal { inner: Vec::new() }
    }

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

    pub fn push(&mut self, freq: usize, len_us: f64) {
        self.inner.push(Component { freq, len_us });
    }
}

pub trait SSTVMode {
    fn new() -> Self;
    fn encode(&mut self, image: DynamicImage) -> Signal;
    fn decode(&mut self, audio: &Vec<f32>) -> DecodeResult {
        todo!()
    }
    fn get_image(&self) -> DynamicImage;
}

pub fn within_50hz(a: f64, b: f64) -> bool {
    (a - b).abs() < 250.
}

pub struct DSPOut {
    pub inner: Vec<f64>,
    pos: usize,
    time_pos: f64,
}

impl DSPOut {
    pub fn new(from: &Vec<f64>) -> DSPOut {
        DSPOut {
            inner: from.clone(),
            pos: 0,
            time_pos: 0.,
        }
    }

    pub fn take_while_frq(&mut self, frq: f64) -> Option<()> {
        let mut at = 0;
        for i in self.pos..self.inner.len() {
            if !within_50hz(*self.inner.get(i)?, frq) {
                at = i;
                break;
            }
        }

        self.pos = at;
        Some(())
    }

    pub fn take_till_frq(&mut self, frq: f64) -> Option<()> {
        let mut at = 0;
        for i in self.pos..self.inner.len() {
            if within_50hz(*self.inner.get(i)?, frq) {
                at = i;
                break;
            }
        }

        self.pos = at;
        Some(())
    }

    pub fn take_us(&mut self, us: f64) -> Option<f64> {
        let total_samples = us_to_n_samples(us);

        let mut sum = 0.;

        for i in self.pos..(self.pos + total_samples) {
            sum += self.inner.get(i)?;
        }

        self.pos += total_samples;

        Some(sum / (total_samples as f64))
    }

    pub fn set_to(&mut self, pos: usize) {
        self.pos = pos;
    }

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

pub enum DecodeResult {
    Finished(DynamicImage),
    Partial(DynamicImage),
    NoneFound,
}
