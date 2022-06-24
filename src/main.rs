mod file_ops;
mod texture_formats;

use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::Parser;
use file_ops::{load_file, parse_file};
use image::{ColorType, ImageFormat};
use texture_formats::SupportedTextureFormats;

use crate::file_ops::generate_timestamped_filename;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        short,
        long,
        value_parser,
        requires = "input",
        default_value = "input.csv"
    )]
    input_file: PathBuf,

    #[clap(short, long, requires = "input", value_parser)]
    output_file: Option<PathBuf>,

    /// The buffer dimensions. Must be formatted as WIDTHxHEIGHT
    #[clap(group = "input", value_parser)]
    dimensions: Option<String>,

    /// The input texture format
    #[clap(short, long, requires = "input", value_parser, default_value = "Rgba8")]
    texture_format: SupportedTextureFormats,

    /// Lists the currently supported texture formats
    #[clap(long, action)]
    list_supported_texture_formats: bool,

    /// For use with non-indexed buffers.
    #[clap(long)]
    no_index: bool,
}

fn main() {
    let total_time = Instant::now();
    let args = Args::parse();

    if args.list_supported_texture_formats {
        SupportedTextureFormats::print_supported_formats();
    } else if let Some(args_dimensions) = args.dimensions {
        let (width, height) = parse_dimensions(args_dimensions);

        let (image_format, output_filename) = match args.output_file {
            Some(output_file) => match output_file.extension() {
                Some(path) => match ImageFormat::from_extension(path) {
                    Some(image_format) => match image_format.can_write() {
                        true => (image_format, output_file),
                        false => panic!("Unsupported image format {:?}", image_format),
                    },
                    None => panic!("Unsupported Image format"),
                },
                None => {
                    let output = output_file.join(PathBuf::from(generate_timestamped_filename()));
                    (ImageFormat::Png, output)
                }
            },
            None => (
                ImageFormat::Png,
                PathBuf::from(generate_timestamped_filename()),
            ),
        };

        let (output_format_length, output_color_type) = match image_format {
            ImageFormat::Pnm => (3, ColorType::Rgb8),
            _ => (4, ColorType::Rgba8)
        };

        let input_path = Path::new(&args.input_file);
        let input_string = load_file(input_path).unwrap();
        let output_vec = parse_file(
            input_string,
            !args.no_index,
            args.texture_format,
            output_format_length,
        );

        let output_time = Instant::now();

        image::save_buffer(
            output_filename,
            &output_vec,
            width,
            height,
            output_color_type,
        )
        .unwrap();

        println!("Time to save image: {:?}", output_time.elapsed());
        println!("Total time: {:?}", total_time.elapsed());
    }
}

fn parse_dimensions(args_dimensions: String) -> (u32, u32) {
    let mut split = args_dimensions.split('x');
    let width = split
        .next()
        .expect("Failed to parse the X dimension")
        .parse::<u32>()
        .expect("Failed to parse the X dimension");
    let height = split
        .next()
        .expect("Failed to parse the Y dimension")
        .parse::<u32>()
        .expect("Failed to parse the Y dimension");
    if width <= 0 || height <= 0 {
        panic!("Error: Invalid dimensions")
    }
    (width, height)
}
