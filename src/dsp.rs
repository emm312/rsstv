use core::f64;

use num_complex::Complex64;

use crate::SAMPLE_RATE;

pub fn quadrature_demod(samples: &Vec<f64>, last: Complex64) -> (Vec<f64>, Complex64) {
    let hilbert = hilbert_transform::hilbert(&samples);

    let mut prv = last;

    let mut ret = Vec::new();

    for sample in &hilbert {
        ret.push((prv.conj() * sample).arg() * (SAMPLE_RATE as f64 / (f64::consts::TAU)));

        prv = *sample;
    }

    (ret, prv)
}
