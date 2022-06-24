use std::fs::File;
use std::io::Result as IoResult;
use std::io::{BufReader, Read};
use std::path::Path;
use std::time::{Instant, SystemTime};

use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

use crate::texture_formats::SupportedTextureFormats;

pub fn generate_timestamped_filename() -> String {
    // Really ugly formatting what I want the output file to be.
    // Millisecond precision is probably good enough.
    let formatted_now = humantime::format_rfc3339_millis(SystemTime::now());
    let mut str_now = String::from("output-");
    str_now.push_str(&formatted_now.to_string());
    // Replace potentially problematic characters
    let str_now = str_now.replace(':', "-");
    let mut str_now = str_now.replace('.', "_");
    str_now.push_str(".png");
    str_now
}

pub fn load_file(path: &Path) -> IoResult<String> {
    let start = Instant::now();
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("Load file duration: {:?}", start.elapsed());
    Ok(contents)
}

pub fn parse_file(
    input: String,
    is_indexed: bool,
    input_format: SupportedTextureFormats,
    output_format_length: usize,
) -> Vec<u8> {
    let start = Instant::now();
    let texture_format_length = input_format.get_buffer_len();

    let indices = if texture_format_length == 3 || output_format_length == 3 {
        input_format.get_rgb_indices()[..3].to_owned()
    } else if texture_format_length == 4 && output_format_length == 4 {
        input_format.get_rgb_indices()[..4].to_owned()
    } else {
        panic!("Failed to parse file. Unsupported texture format.");
    };

    let output_vec: Vec<Vec<u8>> = input
        .par_lines()
        .filter_map(|line| {
            let mut temp_vec = if output_format_length == 4 {
                vec![0, 0, 0, 255]
            } else if output_format_length == 3 {
                vec![0, 0, 0]
            } else {
                panic!("Woah");
            };
            let mut count = 0;
            let mut line_iter = line.split(',');
            if is_indexed {
                line_iter.next();
            }

            indices.iter().for_each(|&index| {
                if let Some(num_str) = line_iter.next() {
                    if let Ok(mut num) = num_str.trim().parse::<f32>() {
                        num *= 255.;
                        temp_vec[index] = num as u8;
                        count += 1;
                    }
                }
            });

            // Make sure we aren't pushing empty lines
            if count == texture_format_length {
                Some(temp_vec)
            } else {
                None
            }
        })
        .collect();

    let output_vec: Vec<u8> = output_vec.into_iter().flatten().collect();

    println!("parse file duration: {:?}", start.elapsed());

    output_vec
}
