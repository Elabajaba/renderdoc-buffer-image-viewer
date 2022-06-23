use std::str::FromStr;

use strum::{Display, EnumIter, IntoEnumIterator};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureFormatError {
    #[error("Texture format `{0}` is either unsupported or invalid.")]
    UnsupportedTextureFormat(String),
}

#[derive(Clone, Debug, Display, EnumIter)]
pub enum SupportedTextureFormats {
    Rgba8,
    Bgra8,
    Rgb8,
    Bgr8,
}

impl SupportedTextureFormats {
    pub fn get_vec4_indices(&self) -> Vec<usize> {
        match self {
            SupportedTextureFormats::Rgba8 => vec![0, 1, 2, 3],
            SupportedTextureFormats::Bgra8 => vec![2, 1, 0, 3],
            SupportedTextureFormats::Rgb8 => vec![0, 1, 2],
            SupportedTextureFormats::Bgr8 => vec![2, 1, 0],
        }
    }

    pub fn get_buffer_len(&self) -> usize {
        match self {
            SupportedTextureFormats::Rgba8 => 4,
            SupportedTextureFormats::Bgra8 => 4,
            SupportedTextureFormats::Rgb8 => 3,
            SupportedTextureFormats::Bgr8 => 3,
        }
    }

    pub fn print_supported_formats() {
        for format in SupportedTextureFormats::iter() {
            println!("{}", format);
        }
    }
}

impl Default for SupportedTextureFormats {
    fn default() -> Self {
        Self::Rgba8
    }
}

impl FromStr for SupportedTextureFormats {
    type Err = TextureFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rgba8" => Ok(SupportedTextureFormats::Rgba8),
            "bgra8" => Ok(SupportedTextureFormats::Bgra8),
            "rgb8" => Ok(SupportedTextureFormats::Rgb8),
            "bgr8" => Ok(SupportedTextureFormats::Bgr8),
            _ => Err(TextureFormatError::UnsupportedTextureFormat(s.to_string())),
        }
    }
}
