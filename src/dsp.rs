use core::f64;

use num_complex::Complex64;

use crate::SAMPLE_RATE;

pub fn quadrature_demod(samples: &Vec<i16>) -> Vec<f64> {
    let mut ret = Vec::new();

    let hilbert = hilbert(samples);

    let mut prv = Complex64::new(0., 0.);

    for sample in hilbert {
        ret.push((prv.conj() * sample).arg() * (SAMPLE_RATE as f64 / f64::consts::TAU));

        prv = sample;
    }
    ret
}

fn hilbert(samples: &Vec<i16>) -> Vec<Complex64> {
    let mut kernel = (0..65)
        .map(|t| 1. / (f64::consts::PI * t as f64))
        .collect::<Vec<f64>>();

    let complex = convolve(&samples, &kernel);

    complex
        .iter()
        .map(|n| *n)
        .zip(samples.iter().map(|n| *n as f64))
        .map(|(a, b)| Complex64 { re: a, im: b })
        .collect()
}

fn convolve(samples: &Vec<i16>, kernel: &Vec<f64>) -> Vec<f64> {
    let mut ret = Vec::new();

    for i in 0..samples.len() {
        let mut sum = 0.;
        for (j, kernel_elem) in kernel.iter().enumerate() {
            sum += (*samples.iter().nth(i + j).unwrap_or(&0) as f64) * kernel_elem;
        }
        ret.push(sum);
    }

    ret
}
