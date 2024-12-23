extern crate image;
extern crate webp;

use image::ImageOutputFormat::{Jpeg, Png};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use webp::Encoder;

pub mod image_optimizer {
    use super::*;

    pub fn compress_jpeg(input_path: &str, output_path: &str, quality: u8) -> Result<(), String> {
        compress_image(input_path, output_path, Jpeg(quality))
    }

    pub fn compress_png(input_path: &str, output_path: &str) -> Result<(), String> {
        compress_image(input_path, output_path, Png)
    }

    pub fn compress_webp(input_path: &str, output_path: &str, quality: f32) -> Result<(), String> {
        let input_file = File::open(input_path).map_err(|e| format!("Failed to open input image at {}: {}", input_path, e))?;
        let input = image::load(BufReader::new(input_file), image::ImageFormat::WebP).map_err(|e| format!("Error loading image at {}: {}", input_path, e))?;

        let webp_encoder = Encoder::from_image(&input).map_err(|_| "Error converting image to WebP encoder".to_string())?;
        let webp_data = webp_encoder.encode(quality).map_err(|_| "Error encoding image with provided quality".to_string())?;

        std::fs::write(output_path, webp_data).map_err(|e| format!("Failed to write output image to {}: {}", output_path, e))?;

        Ok(())
    }

    fn compress_image(input_path: &str, output_path: &str, format: image::ImageOutputFormat) -> Result<(), String> {
        let img = image::open(input_path).map_err(|e| format!("Failed to open input image at {}: {}", input_path, e))?;
        
        img.save_with_format(output_path, format).map_err(|e| format!("Failed to save output image to {}: {}", output_path, e))?;
        Ok(())
    }
}