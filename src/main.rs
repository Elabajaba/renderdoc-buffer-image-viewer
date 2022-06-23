mod texture_formats;
mod vec4;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use clap::Parser;
use texture_formats::SupportedTextureFormats;
use vec4::Vec4;

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
    let args = Args::parse();

    if args.list_supported_texture_formats {
        SupportedTextureFormats::print_supported_formats();
    } else if let Some(args_dimensions) = args.dimensions {
        let (x, y): (u32, u32) = {
            let mut split = args_dimensions.split('x');
            let x = split
                .next()
                .expect("Failed to parse the X dimension")
                .parse::<u32>()
                .expect("Failed to parse the X dimension");
            let y = split
                .next()
                .expect("Failed to parse the Y dimension")
                .parse::<u32>()
                .expect("Failed to parse the Y dimension");
            if x <= 0 || y <= 0 {
                panic!("Error: Invalid dimensions")
            }
            (x, y)
        };

        let input_path = Path::new(&args.input_file);
        let output_vec = load_and_parse_file(input_path, !args.no_index, args.texture_format)
            .expect("Failed to parse or read file");

        let output_filename = match args.output_file {
            Some(filename) => filename,
            None => {
                // Really ugly formatting what I want the output file to be.
                let now = SystemTime::now();
                // Millisecond precision is probably good enough.
                let formatted_now = humantime::format_rfc3339_millis(now);
                let mut str_now = String::from("output-");
                str_now.push_str(&formatted_now.to_string());
                // Replace potentially problematic characters
                let str_now = str_now.replace(':', "-");
                let mut str_now = str_now.replace('.', "_");
                str_now.push_str(".ppm");
                PathBuf::from(str_now)
            }
        };

        write_file(Path::new(&output_filename), &output_vec, x, y).expect("Failed to write file");
    }
}

fn load_and_parse_file(
    path: &Path,
    is_indexed: bool,
    format: SupportedTextureFormats,
) -> std::io::Result<Vec<Vec4>> {
    let mut output_vec = Vec::new();

    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    // How many values do we parse
    let texture_format_length = format.get_buffer_len();

    let mut temp_vec = Vec4::default();

    for line in buf_reader.lines() {
        if let Ok(line) = line {
            let mut line_iter = line.split(',');
            if is_indexed {
                line_iter.next();
            }
            let mut count = 0;

            for index in format.get_vec4_indices() {
                if let Some(num_str) = line_iter.next() {
                    if let Ok(mut num) = num_str.trim().parse::<f32>() {
                        num *= 255.;
                        temp_vec[index] = num as i32;

                        count += 1;
                    }
                }
            }

            if count == texture_format_length {
                output_vec.push(temp_vec);
            }

            temp_vec = Vec4::default();
        }
    }

    Ok(output_vec)
}

fn write_file(path: &Path, contents: &[Vec4], x: u32, y: u32) -> std::io::Result<()> {
    use std::io::Write;

    // The PPM header
    let header = format!("P3\n# My comment\n{} {}\n255\n", x, y);

    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write(header.as_bytes())?;
    for num in contents {
        writeln!(buf_writer, "{}", num.to_rgb_string())?;
    }
    Ok(())
}
