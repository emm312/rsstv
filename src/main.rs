use std::sync::mpsc;

#[cfg(feature = "cli")]
use cpal::{
    SampleRate, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use image::{ImageFormat, ImageReader};
use rsstv::{
    SAMPLE_RATE,
    common::{DecodeResult, SSTVMode},
    martinm1::MartinM1,
};

use hound::{WavReader, WavSpec, WavWriter};

#[cfg(feature = "cli")]
use clap::Parser;

/// CLI argument struct, powered by clap
#[cfg(feature = "cli")]
#[derive(Parser)]
struct Args {
    /// Image or audio file to encode/decode from
    #[clap()]
    input_file: Option<String>,

    /// The file to write the output WAV to
    #[clap(short, long, default_value = "out.wav")]
    ouput_file: String,

    /// Decode flag - will encode if not passed
    #[clap(short, long)]
    decode: bool,

    /// Decode from the microphone audio input
    #[clap(short, long)]
    mic: bool,
}

#[cfg(feature = "cli")]
fn main() {
    use hound::SampleFormat;

    let args = Args::parse();

    let mut mode = MartinM1::new();

    if args.decode {
        if !args.mic {
            // If decoding from a WAV file, load samples and decode all at once.
            // Can also make the samples vec into an iterator to split into chunks,
            // useful for testing live decodes.
            let mut reader = WavReader::open(args.input_file.unwrap()).unwrap();
            assert_eq!(reader.spec().sample_rate, SAMPLE_RATE as u32);

            let samples: Vec<f32>;

            match (reader.spec().sample_format, reader.spec().bits_per_sample) {
                (SampleFormat::Float, 32) => {
                    samples = reader.samples().map(|s: Result<f32, _>| s.unwrap()).collect()
                }
                (SampleFormat::Int, 16) => {
                    samples = reader.samples().map(|s: Result<i16, _>| s.unwrap() as f32).collect()
                }
                _ => todo!()
            }

            let out = mode.decode(&samples.to_vec());

            match out {
                DecodeResult::Finished(image) | DecodeResult::Partial(image) => {
                    image.save_with_format("out.png", ImageFormat::Png).unwrap()
                }
                DecodeResult::NoneFound => println!("No image found"),
            }
        } else {
            let mut decoder = MartinM1::new();

            // If decoding from the mic, detect the default microphone
            let host = cpal::default_host();
            let device = host.default_input_device().unwrap();

            println!("using device {:?}", device.name().unwrap());

            let default_config = device.default_input_config().unwrap();

            // Set sample rate to 44.1KHz
            // TODO: make it work at other sample rates
            let mut config: StreamConfig = default_config.into();
            config.sample_rate = SampleRate(SAMPLE_RATE as u32);

            // Multithread channels, `rx` will blockingly wait for a chunk of data
            let (tx, rx) = mpsc::channel();

            let stream = device
                .build_input_stream(
                    &config,
                    move |data: &[f32], _| {
                        // When data is received, send it over the tx channel to the main thread
                        tx.send(data.to_vec()).unwrap();
                    },
                    |err| println!("{:#?}", err),
                    None,
                )
                .unwrap();
            // Start gathering data in another thread
            stream.play().unwrap();

            loop {
                // Main thread logic:
                let mut buf = Vec::new();

                // Get data from streaming thread, accumulating into a bigger vec of at least 100k samples
                // this makes the live decode not be the bottleneck in real time decodes, as it doesn't
                // have to do the DSP processing as many times which takes roughly 100ms (which happens
                // every time we call the decode fn) (filtering, quadrature demod)
                // This adds up fast when sending buffers of just 512 samples.
                // TODO: make it faster and send samples directly without accumulating
                while buf.len() <= 100_000 {
                    let mut received = rx.recv().unwrap();
                    buf.append(&mut received);
                }

                let decode = decoder.decode(&buf);

                // Save image every time we call decode
                if let DecodeResult::Partial(ref image) = decode {
                    image.save_with_format("out.png", ImageFormat::Png).unwrap();
                }
                if let DecodeResult::Finished(ref image) = decode {
                    image.save_with_format("out.png", ImageFormat::Png).unwrap();
                    break;
                }
            }

            // End streaming from the mic
            drop(stream);
            println!("Finished decoding");
        }
    } else {
        // Else, we encode the image to audio!

        // Open the image file
        let reader = ImageReader::open(args.input_file.unwrap())
            .unwrap()
            .decode()
            .unwrap();

        // Encode
        let signal = mode.encode(reader);

        // And write
        let written: &[f32] = &signal.to_samples();

        let spec = WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE as u32,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut writer = WavWriter::create(args.ouput_file, spec).unwrap();

        for elem in written {
            writer.write_sample(*elem).unwrap();
        }

        writer.finalize().unwrap();
    }
}

// Here to stop the rust compiler complaining that there is no main function with wasm target
#[cfg(not(feature = "cli"))]
fn main() {}
