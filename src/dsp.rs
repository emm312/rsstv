use core::f64;

use num_complex::Complex64;

use crate::SAMPLE_RATE;

pub fn quadrature_demod(samples: &Vec<i16>) -> Vec<f64> {
    let mut ret = Vec::new();

    let samples_f64 = samples.iter().map(|s| *s as f64).collect::<Vec<f64>>();

    let hilbert = hilbert_transform::hilbert(&samples_f64);

    let mut prv = Complex64::new(0., 0.);

    for sample in hilbert {
        ret.push((prv.conj() * sample).arg() * (SAMPLE_RATE as f64 / (f64::consts::TAU)));

        prv = sample;
    }

    ret
}
