use std::path::Path;

use eframe::{NativeOptions, egui::ViewportBuilder};
use image::{ImageFormat, ImageReader};
use rsstv::{
    SAMPLE_RATE,
    app::RSSTV,
    common::{SSTVMode, Signal},
    martinm1::MartinM1,
};

use clap::Parser;
use wavers::Wav;

#[derive(Parser)]
struct Args {
    #[clap()]
    input_file: String,

    #[clap(short, long, default_value = "out.wav")]
    ouput_file: String,

    #[clap(short, long)]
    decode: bool,
}

fn main() {
    let args = Args::parse();

    let mut mode = MartinM1::new();

    if args.decode {
        let samples = Wav::from_path(args.input_file).unwrap().read().unwrap();

        let out = mode.decode(&samples.to_vec());

        out.save_with_format("out.png", ImageFormat::Png).unwrap();
    } else {
        let reader = ImageReader::open(args.input_file)
            .unwrap()
            .decode()
            .unwrap();

        let signal = mode.encode(reader);

        //let mut signal = Signal::new();
        //signal.push(1200, 50000.);

        let written: &[i16] = &signal.to_samples().convert();
        wavers::write(Path::new(&args.ouput_file), written, SAMPLE_RATE as i32, 1).unwrap();
    }
}

//fn main() {
//    let options = NativeOptions {
//        viewport: ViewportBuilder::default()
//            .with_inner_size([640.0, 240.0])
//            .with_drag_and_drop(true),
//        ..Default::default()
//    };
//
//    eframe::run_native(
//        "RSSTV",
//        options,
//        Box::new(|cc| {
//            egui_extras::install_image_loaders(&cc.egui_ctx);
//            Ok(Box::<RSSTV>::default())
//        }),
//    )
//    .unwrap();
//}
//
