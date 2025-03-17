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
    fn decode(&mut self, audio: &Vec<i16>) {
        todo!()
    }
    fn get_image(&self) -> DynamicImage;
}
