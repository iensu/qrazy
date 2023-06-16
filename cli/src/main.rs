use anyhow::Result;
use clap::Parser;
use image::png::PngEncoder;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input to turn into a QR code
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let code = qr_gen::generate(&args.input)?.scale(20);

    let file = std::fs::File::create("./qr-code.png")?;

    let png_encoder = PngEncoder::new(file);
    let width = code.width;
    let data: Vec<u8> = code
        .bits
        .into_iter()
        .map(|x| x + x * (u8::MAX - 1))
        .collect();
    png_encoder.encode(&data, width, width, image::ColorType::L8)?;

    Ok(())
}
