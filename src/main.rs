use clap::Parser;
use ascii_converter::{ASCIIImage, Options, ASCII};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The image to convert
    #[arg(short, long)]
    image: String,

    /// The output image
    #[arg(short, long)]
    output: String,

    /// The width of the output image
    #[arg(short, long)]
    width: Option<u32>,

    /// The height of the output image
    #[arg(short= 'H', long)]
    height: Option<u32>,

    /// The color of the output image
    #[arg(short = 'C', long, default_value = "false")]
    color: String,

    /// The charsets to use
    #[arg(short, long, default_value = "default")]
    charsets: String,
}

fn main() -> Result<(), image::ImageError> {
    let args = Args::parse();

    let mut ascii_image = ASCIIImage::new(args.image, Options {
        width: args.width,
        height: args.height,
        color: args.color.as_str() == "true",
        charsets: args.charsets.as_str(),
    });

    let image = ASCII::reader(&mut ascii_image);
    let greyscale = ASCII::convert_to_greyscale(&mut ascii_image, &image);
    let ascii = ASCII::convert_to_ascii(&ascii_image, greyscale);
    ASCII::save_image(&ascii_image ,ascii, &"./output.png")?;
    Ok(())
}
