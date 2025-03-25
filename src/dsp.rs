use core::f64;

use num_complex::Complex64;

use crate::SAMPLE_RATE;

/// This function does various DSP operations on the `samples` vec
/// 
/// First it performs a hilbert transform, using the result to do a quadrature demod.
/// 
/// # Math behind the quadrature demod:
/// IQ samples = e^(iθ)
/// 
/// Z1 * Z2 = r1 * r2 * e^(i(θ1 + θ2)) (de moivre's theorem)
/// 
/// therefore
/// 
/// Z1 * conj(Z2) = r1 * r2 * e^(i(θ1 - θ2))
/// 
/// making
/// 
/// arg(Z1 * conj(Z2)) the phase difference between Z1 and Z2
/// 
/// This means that the principal argument of current sample times the conjugate
/// of the previous sample will equal the phase difference between the two,
/// which can be converted to a frequency reading by multiplying with:
/// 
/// SAMPLE_RATE / (2 * pi)
/// 
/// This means we can get a continuous frequency measurement over all the samples
pub fn quadrature_demod(samples: &Vec<f64>) -> Vec<f64> {
    let hilbert = hilbert_transform::hilbert(&samples);

    let mut prv = Complex64::ZERO;

    // NOTE: Remove this allocation
    let mut ret = Vec::with_capacity(samples.len());

    for sample in &hilbert {
        ret.push((prv.conj() * sample).arg() * (SAMPLE_RATE as f64 / (f64::consts::TAU)));

        prv = *sample;
    }

    ret
}
