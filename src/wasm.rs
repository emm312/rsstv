use std::io::{BufReader, Cursor};

use crate::{
    common::{DecodeResult, SSTVMode},
    martinm1::MartinM1,
};
use image::{DynamicImage, ImageBuffer, ImageDecoder, ImageReader, Rgb, codecs::png::PngDecoder};
use wasm_bindgen::prelude::*;

/// A struct providing easy to use JS bindings for the `MartinM1` struct
///
/// TODO: Generalise this type to work with any `impl SSTVMode`
#[wasm_bindgen]
pub struct SSTVDecoderWASM {
    inner: MartinM1,
}

#[wasm_bindgen]
impl SSTVDecoderWASM {
    #[wasm_bindgen]
    pub fn new() -> SSTVDecoderWASM {
        SSTVDecoderWASM {
            inner: MartinM1::new(),
        }
    }

    #[wasm_bindgen]
    pub fn decode(&mut self, buf: &[f32]) -> Option<Vec<u8>> {
        let result = self.inner.decode(&buf);

        match result {
            DecodeResult::Finished(image) | DecodeResult::Partial(image) => {
                Some(image.as_bytes().to_vec())
            }
            DecodeResult::NoneFound => None,
        }
    }

    #[wasm_bindgen]
    pub fn encode(&mut self, image: Vec<u8>) -> Vec<f32> {
        let image = ImageReader::new(Cursor::new(image));

        let result = self.inner.encode(image.decode().unwrap());

        result.to_samples().to_vec()
    }
}
