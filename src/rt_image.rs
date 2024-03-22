use image::{ImageBuffer, Rgb};

use crate::color::Color;

#[derive(Clone)]
pub struct RtImage {
    image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    image_width: u32,
    image_height: u32,
}

impl RtImage {
    pub fn new(image_filename: &str) -> RtImage {
        let bytes = match image::open(image_filename) {
            Ok(data) => data.to_rgb8(),
            Err(_) => {
                eprintln!("Could not open image.");
                panic!()
            }
        };

        RtImage {
            image_height: bytes.height(),
            image_width: bytes.width(),
            image: bytes,
        }
    }

    pub fn height(&self) -> u32 {
        self.image_height
    }

    pub fn width(&self) -> u32 {
        self.image_width
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> Color {
        let x = x.clamp(0, self.image_width - 1);
        let y = (self.image_height - y - 1).clamp(0, self.image_height - 1);

        let p = self.image.get_pixel(x, y).0;
        let (r, g, b) = (p[0] as f64, p[1] as f64, p[2] as f64);
        let color_scale = 1.0 / 255.0;

        Color::new(r * color_scale, g * color_scale, b * color_scale)
    }
}

