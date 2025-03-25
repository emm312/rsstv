/// `common` contains common code used by every mode and
/// the traits required to implement them.
pub mod common;

/// Functions required to process the incoming samples into
/// a time-frequency domain signal, suitable for decoding
pub mod dsp;

/// The Martin M1 mode transcoder
pub mod martinm1;

/// Wasm glue code
#[cfg(feature = "wasm")]
pub mod wasm;

pub const SAMPLE_RATE: usize = 44100;
