use image::{ImageReader, DynamicImage, GrayImage, GenericImageView, GenericImage, Rgba, ColorType, RgbImage, Rgb};
use imageproc::drawing::draw_text_mut;
use ansi_term::Color;
use ab_glyph::{FontRef, PxScale, Font, ScaleFont};


mod charsets;

pub struct ColoredChar {
    ch: char,
    color: Rgb<u8>,
}

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub grey: u8,
    pub position: Vec<(u32, u32)>,
}

struct Pixels {
    pixels: Vec<Pixel>,
}

impl Pixels {
    fn get_pixel(&self, x: u32, y: u32) -> &Pixel {
        self.pixels.iter().find(|pixel| pixel.position.contains(&(x, y))).unwrap()
    }
}

pub trait ASCII {
    fn reader(&mut self) -> DynamicImage;
    fn convert_to_greyscale(&mut self, image: &DynamicImage) -> GrayImage;
    fn convert_to_ascii(&self, image: GrayImage) -> Vec<Vec<ColoredChar>>;
    fn save_image(&self, ascii: Vec<Vec<ColoredChar>>, output_path: &str) -> Result<(), image::ImageError>;
}


pub struct ASCIIImage<'a> {
    image_path: String,
    options: Options<'a>,
    pixels: Pixels,
    nb_chars_per_line: u32,
    nb_chars_per_column: u32,
}

pub struct Options<'a> {
    pub columns: Option<u32>,
    pub lines: Option<u32>,
    pub color: bool,
    pub charsets: &'a str,
}

impl <'a>ASCIIImage<'a> {
    pub fn new(image_path: String, options: Options<'a>) -> ASCIIImage {
        ASCIIImage {
            image_path,
            options,
            pixels: Pixels { pixels: Vec::new() },
            nb_chars_per_column: 0,
            nb_chars_per_line: 0,
        }
    }

    pub fn pixel_to_greyscale(&self, pixel: Rgba<u8>) -> u8 {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        // Apply the sRGB color space conversion formula: https://en.wikipedia.org/wiki/Grayscale
        ((0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) / 3.0) as u8
    }

    pub fn save_greyscale(&mut self, output_path: &str) -> Result<(), image::ImageError> {
        let image = self.reader();
        let greyscale_image = self.convert_to_greyscale(&image);
        greyscale_image.save(output_path)
    }

    pub fn find_char<'b>(&self, charsets: &[&'b str], pixel: u8) -> &'b str {
        let index = ((charsets.len() - 1) * pixel as usize) / 255 as usize;
        charsets[index]
    }

    
    pub fn resize(&mut self, image: DynamicImage) -> DynamicImage {
        let (img_width, img_height) = image.dimensions();
        println!("Image dimensions: width: {}, height: {}", img_width, img_height);

        let (char_aspect_ratio, char_height, char_width) = self.get_char_aspect_ratio('W');

        // Image aspect ratio
        let img_aspect_ratio = img_width as f32 / img_height as f32;

        // Effective aspect ratio
        let effective_aspect_ratio = img_aspect_ratio * char_aspect_ratio;
        println!("Image aspect ratio: {}, Effective aspect ratio: {}", img_aspect_ratio, effective_aspect_ratio);

        // Decide on the number of characters per line and per column
        match (self.options.columns, self.options.lines) {
            (Some(width_chars), Some(height_chars)) => {
                self.nb_chars_per_line = width_chars;
                self.nb_chars_per_column = height_chars;
            },
            (Some(width_chars), None) => {
                self.nb_chars_per_line = width_chars;
                self.nb_chars_per_column = (self.nb_chars_per_line as f32 * effective_aspect_ratio).round() as u32;
            },
            (None, Some(height_chars)) => {
                self.nb_chars_per_column = height_chars;
                self.nb_chars_per_line = (self.nb_chars_per_column as f32 / effective_aspect_ratio).round() as u32;
            },
            (None, None) => {
                // Default values if neither width nor height is specified
                self.nb_chars_per_line = 80;
                self.nb_chars_per_column = (self.nb_chars_per_line as f32 * effective_aspect_ratio).round() as u32;
            }
        }

        // Resize the image to match the character grid dimensions
        let resized_image = image.resize_exact(
            self.nb_chars_per_line,
            self.nb_chars_per_column,
            image::imageops::FilterType::Nearest
        );

        resized_image
    }


    fn get_char_aspect_ratio(&self, ch: char) -> (f32, f32, f32) {
        let scale = PxScale::from(12.0);
        
        let font_data = include_bytes!("fonts/Anonymous Pro.ttf");
        let font = FontRef::try_from_slice(font_data).expect("Failed to load font");
        
        let scaled_font = font.into_scaled(scale.y);
        
        let glyph_id = scaled_font.glyph_id(ch);
        let char_width = scaled_font.h_advance(glyph_id);
        
        let ascent = scaled_font.ascent();
        let descent = scaled_font.descent();
        let line_gap = scaled_font.line_gap();
        let char_height = ascent - descent + line_gap;
       
        (char_width as f32 / char_height as f32, char_height as f32, char_width as f32)
    }

}

impl ASCII for ASCIIImage<'_> {
    fn reader(&mut self) -> DynamicImage {
        let image = ImageReader::open(self.image_path.clone()).unwrap().decode().unwrap();
        self.resize(image)
    }

    fn convert_to_greyscale(&mut self, image: &DynamicImage) -> GrayImage {
        let (width, height) = image.dimensions();
        let mut greyscale_image = GrayImage::new(width, height);
        for x in 0..width {
            for y in 0..height {
                let pixel = image.get_pixel(x, y);
                let greyscale = self.pixel_to_greyscale(pixel);
                self.pixels.pixels.push(Pixel {
                    r: pixel[0],
                    g: pixel[1],
                    b: pixel[2],
                    a: pixel[3],
                    grey: greyscale,
                    position: vec![(x, y)]
                });
                greyscale_image.put_pixel(x, y, image::Luma([greyscale]));
            }
        }
        greyscale_image.save("greyscale.png");
        greyscale_image
    }


        fn convert_to_ascii(&self, image: GrayImage) -> Vec<Vec<ColoredChar>> {
        let chars = charsets::DEFAULT;
        let mut ascii_art = Vec::new();

        for y in 0..image.height() {
            let mut line = Vec::new();
            for x in 0..image.width() {
                let pixel = self.pixels.get_pixel(x, y);
                let ch = self.find_char(chars, pixel.grey).chars().next().unwrap();
                let color = Rgb([pixel.r, pixel.g, pixel.b]);

                line.push(ColoredChar { ch, color });

                // Terminal output with color
                if self.options.color {
                    let ansi_color = Color::RGB(pixel.r, pixel.g, pixel.b);
                    print!("{}", ansi_color.paint(ch.to_string()));
                } else {
                    print!("{}", ch);
                }
            }
            ascii_art.push(line);
            println!();
        }
        ascii_art
    }

    fn save_image(&self, ascii_art: Vec<Vec<ColoredChar>>, output_path: &str) -> Result<(), image::ImageError> {
        let scale = PxScale::from(12.0);
        let line_height = scale.y.ceil() as u32;
        let num_lines = ascii_art.len();

        // Load the font and create a scaled version
        let font_data = include_bytes!("fonts/Anonymous Pro.ttf");
        let font = FontRef::try_from_slice(font_data).expect("Failed to load font");
        let glyph = font.clone().glyph_id('W');
        let scaled_font = font.clone().into_scaled(scale.y);

        // Calculate the width of each line and the maximum width
        let mut max_line_width = 0.0;
        let mut line_widths = Vec::new();

        for line in &ascii_art {
            let line_width: f32 = line.len() as f32 * scaled_font.h_advance(glyph);
            line_widths.push(line_width);
            if line_width > max_line_width {
                max_line_width = line_width;
            }
        }

        let width = max_line_width.ceil() as u32;
        let height = (num_lines as u32) * line_height;

        let mut img = RgbImage::new(width, height);

        // Fill the background with white
        for pixel in img.pixels_mut() {
            *pixel = Rgb([255, 255, 255]);
        }

        // Draw each character with its color
        for (y_idx, line) in ascii_art.iter().enumerate() {
            let y = y_idx as f32 * scale.y;
            let mut x = 0.0;
            for colored_char in line {
                let ch_str = colored_char.ch.to_string();
                draw_text_mut(&mut img, colored_char.color, x as i32, y as i32, scale, &font, &ch_str);
                x += scaled_font.h_advance(glyph);
            }
        }

        img.save(output_path)?;
        Ok(())
    }
}
