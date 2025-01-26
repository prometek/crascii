use clap::Parser;
use std::borrow::Cow; 
use crascii::{ASCIIImage, Options};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The image to convert
    #[arg(short, long, help = "Path to the input image file to be converted into ASCII art")]
    image: String,

    /// The width of the output image
    #[arg(short = 'w', long, help = "Number of columns (width) for the ASCII output")]
    columns: Option<u32>,

    /// The height of the output image
    #[arg(short= 'H', long, help = "Number of lines (height) for the ASCII output")]
    lines: Option<u32>,

    /// The color of the output image
    /// Enable colored ASCII output
    #[arg(short = 'C', long, help = "Enable colored ASCII output")]
    color: bool,

    /// The charsets to use
    #[arg(short, long, default_value = "default", help = "Character set to use for ASCII conversion")]
    charsets: String,

    /// Output file
    #[arg(short, long, help = "Path to save the ASCII output")]
    output_path: Option<String>,

    /// Print the output image to the console
    #[arg(short, long, help = "Flag to print the ASCII image directly to the console")]
    print: bool,

    /// Font size
    #[arg(short, long, help = "Font size for the ASCII output")]
    font_size: Option<f32>,
}


fn parse_args() -> Result<Args, String> {
    let args = Args::parse();

    if args.image.is_empty() {
        return Err("Image path cannot be empty".to_string());
    }

    if args.output_path.is_none() && !args.print {
        return Err("At least one of the following flags must be set: --output-path or --print".to_string());
    }

    Ok(args)
}

fn main() -> Result<(), image::ImageError> {
    match parse_args() {
        Ok(args) => {
            let mut ascii_image = ASCIIImage::new(args.image, Options {
                columns: args.columns,
                lines: args.lines,
                color: args.color,
                print: args.print,
                charsets: Cow::Owned(args.charsets),
                output_path: Cow::Owned(args.output_path.unwrap_or("".to_string())),
                font_size: args.font_size,
            });
            ascii_image.convert();
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::Parser;

    #[test]
    fn test_parse_args() {
        let args = vec![
            "your_tool_name",
            "--image", "input.png",
            "--output-path", "output.txt",
            "--color",
            "--columns", "80",
            "--lines", "40",
        ];
        let args = Args::parse_from(args);
        assert_eq!(args.image, "input.png");
        assert_eq!(args.output_path, Some("output.txt".to_string()));
        assert!(args.color);
        assert_eq!(args.columns, Some(80));
        assert_eq!(args.lines, Some(40));
    }
}

