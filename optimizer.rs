extern crate image;
extern crate webp;

use image::ImageOutputFormat::{Jpeg, Png};
use image::{DynamicImage, ImageError};
use std::{fs::File, io::{BufReader, Error as IoError}, path::Path};
use webp::Encoder;

pub mod image_optimizer {
    use super::*;

    #[derive(Debug)]
    pub enum ImageOptimizerError {
        IoError(IoError),
        ImageError(ImageError),
        EncodeWebPError(String),
        WriteError(String),
    }

    impl From<IoError> for ImageOptimizerError {
        fn from(err: IoError) -> Self {
            ImageOptimizerError::IoError(err)
        }
    }

    impl From<ImageError> for ImageOptimizerError {
        fn from(err: ImageError) -> Self {
            ImageOptimizerError::ImageError(err)
        }
    }

    pub fn compress_jpeg(input_path: &str, output_path: &str, quality: u8) -> Result<(), ImageOptimizerError> {
        compress_image(input_path, output_path, Jpeg(quality))
    }

    pub fn compress_png(input_path: &str, output_path: &str) -> Result<(), ImageOptimizerError> {
        compress_image(input_path, output_path, Png)
    }

    pub fn compress_webp(input_path: &str, output_path: &str, quality: f32) -> Result<(), ImageOptimizerError> {
        let input_file = File::open(input_path)?;
        let input = image::load(BufReader::new(input_file), image::ImageFormat::WebP)?;

        let webp_encoder = Encoder::from_image(&input).ok_or_else(|| ImageOptimizerError::EncodeWebPError("Error converting image to WebP encoder".to_string()))?;
        let webp_data = webp_encoder.encode(quality).map_err(|_| ImageOptimizerError::EncodeWebPError("Error encoding image with provided quality".to_string()))?;

        std::fs::write(output_path, webp_data).map_err(|e| ImageOptimizerError::WriteError(format!("Failed to write output image to {}: {}", output_path, e)))?;

        Ok(())
    }

    fn compress_image(input_path: &str, output_path: &str, format: image::ImageOutputFormat) -> Result<(), ImageOptimizerError> {
        let img = image::open(input_path)?;
        
        img.save_with_format(output_path, format)?;
        Ok(())
    }
}

impl std::fmt::Display for image_optimizer::ImageOptimizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            image_optimizer::ImageOptimizerError::IoError(err) => write!(f, "I/O Error: {}", err),
            image_optimizer::ImageOptimizerError::ImageError(err) => write!(f, "Image processing error: {}", err),
            image_optimizer::ImageOptimizerError::EncodeWebPError(err) => write!(f, "{}", err),
            image_optimizer::ImageOptimizerError::WriteError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for image_optimizer::ImageOptimizerError {}