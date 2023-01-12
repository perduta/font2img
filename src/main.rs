use clap::Parser;
use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};

fn get_printable_chars(font: &Font, scale: Scale) -> Vec<char> {
    (0..=std::u8::MAX)
        .map(|c| std::char::from_u32(c as _).unwrap())
        .filter(|c| c.is_ascii() && text_size(scale, font, &c.to_string()) != (0, 0))
        .collect()
}

fn max_size(font: &Font, scale: Scale, printable_chars: &[char]) -> u32 {
    printable_chars
        .iter()
        .map(|symbol| text_size(scale, font, &symbol.to_string()))
        .map(|(w, h)| w.max(h))
        .max()
        .unwrap() as _
}

fn draw_characters(
    image: &mut RgbaImage,
    font: &Font,
    scale: Scale,
    printable_chars: &[char],
    max_size: u32,
) {
    for (i, c) in printable_chars.iter().enumerate() {
        let x = (i * max_size as usize) as i32 + (max_size / 2) as i32;
        draw_text_mut(
            image,
            Rgba([255, 255, 255, 255]),
            x,
            0,
            scale,
            font,
            &c.to_string(),
        );
    }
}

fn create_image(font_size: f32, font_path: &str, output_path: &str) {
    let font_bytes = std::fs::read(font_path).unwrap();
    let font = Font::try_from_bytes(&font_bytes).unwrap();
    let scale = Scale {
        x: font_size,
        y: font_size,
    };
    let printable_chars = get_printable_chars(&font, scale);
    let max_size = max_size(&font, scale, &printable_chars);
    let image_width = (max_size as usize * printable_chars.len()) as u32 + (max_size / 2);
    let mut image = DynamicImage::new_rgba8(image_width, max_size).to_rgba8();
    draw_characters(&mut image, &font, scale, &printable_chars, max_size);
    image.save(output_path).unwrap();
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ClapArgs {
    #[arg(short = 's')]
    font_size: f32,

    #[arg(short, long)]
    font_path: String,

    #[arg(short, long)]
    output_path: String,
}

fn main() {
    let args = ClapArgs::parse();
    create_image(args.font_size, &args.font_path, &args.output_path);
}