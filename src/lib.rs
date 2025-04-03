/// # RSSTV
/// RSSTV is a SSTV transcoder written in rust, supporting encoding, decoding
/// live streaming from the microphone.
///
/// ## Current features
/// - Martin M1 transcoding
/// - Microphone streaming
/// - A base to work off to add more modes
///
/// ## Planned features
/// - More modes
/// - Faster decoding (currently takes 100ms per partial decode in a live decode)
/// - A website powered by WASM
///

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
