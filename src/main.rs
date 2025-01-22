use clap::Parser;
use std::borrow::Cow; 
use crascii::{ASCIIImage, Options};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The image to convert
    #[arg(short, long)]
    image: String,

    /// The width of the output image
    #[arg(short = 'w', long)]
    columns: Option<u32>,

    /// The height of the output image
    #[arg(short= 'H', long)]
    lines: Option<u32>,

    /// The color of the output image
    #[arg(short = 'C', long, default_value = "false")]
    color: String,

    /// The charsets to use
    #[arg(short, long, default_value = "default")]
    charsets: String,

    /// Output file
    #[arg(short, long)]
    output_path: String,

    #[arg(short, long)]
    print: String,
}

fn main() -> Result<(), image::ImageError> {
    let args = Args::parse();

    let mut ascii_image = ASCIIImage::new(args.image, Options {
        columns: args.columns,
        lines: args.lines,
        color: args.color.as_str() == "true",
        charsets: Cow::Owned(args.charsets),
        output_path: Cow::Owned(args.output_path),
    });
    ascii_image.convert();
    //let image = ASCII::reader(&mut ascii_image);
    //let greyscale = ASCII::convert_to_greyscale(&mut ascii_image, &image);
    //let ascii = ASCII::convert_to_ascii(&ascii_image, greyscale);
    //ASCII::save_image(&ascii_image ,ascii, &"./output.png")?;
    //ascii_image.convert()?;
    //ascii_image.convert_with_animation(100)?;
    //ascii_image.convert_with_animation(5)?;
    Ok(())
}
