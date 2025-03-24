use std::{path::Path, sync::mpsc, time::Duration};

use cpal::{
    StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use image::{ImageFormat, ImageReader};
use rsstv::{
    SAMPLE_RATE,
    common::{DecodeResult, SSTVMode},
    martinm1::MartinM1,
};

use clap::Parser;
use wavers::Wav;

#[derive(Parser)]
struct Args {
    #[clap()]
    input_file: Option<String>,

    #[clap(short, long, default_value = "out.wav")]
    ouput_file: String,

    #[clap(short, long)]
    decode: bool,

    #[clap(short, long)]
    mic: bool,
}

fn main() {
    let args = Args::parse();

    let mut mode = MartinM1::new();

    if args.decode {
        if !args.mic {
            let samples = Wav::from_path(args.input_file.unwrap())
                .unwrap()
                .read()
                .unwrap();

            let chunks = samples.chunks(512);

            for chunk in chunks {
                let out = mode.decode(&chunk.to_vec());

                match out {
                    DecodeResult::Finished(image) | DecodeResult::Partial(image) => {
                        image.save_with_format("out.png", ImageFormat::Png).unwrap()
                    }
                    DecodeResult::NoneFound => println!("No image found"),
                }
            }


        } else {
            let mut decoder = MartinM1::new();

            let host = cpal::default_host();
            let device = host.default_input_device().unwrap();

            println!("using device {:?}", device.name().unwrap());

            let config = device.default_input_config().unwrap();
            println!("Default input config: {:?}", config);

            let (tx, rx) = mpsc::channel();

            let stream = device
                .build_input_stream(
                    &config.into(),
                    move |data: &[f32], _| {
                        tx.send(data.to_vec()).unwrap();
                    },
                    |err| println!("{:#?}", err),
                    None,
                )
                .unwrap();
            stream.play().unwrap();

            loop {
                let decode = decoder.decode(&rx.recv().unwrap());
                if let DecodeResult::Partial(ref image) = decode {
                    image.save_with_format("out.png", ImageFormat::Png).unwrap();
                }
                if let DecodeResult::Finished(ref image) = decode {
                    image.save_with_format("out.png", ImageFormat::Png).unwrap();
                    break;
                }
            }

            drop(stream);
            println!("Finished decoding");
        }
    } else {
        let reader = ImageReader::open(args.input_file.unwrap())
            .unwrap()
            .decode()
            .unwrap();

        let signal = mode.encode(reader);

        //let mut signal = Signal::new();
        //signal.push(1200, 50000.);

        let written: &[f64] = &signal.to_samples().convert();
        wavers::write(Path::new(&args.ouput_file), written, SAMPLE_RATE as i32, 1).unwrap();
    }
}
