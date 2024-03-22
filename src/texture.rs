use std::sync::Arc;

use crate::{
    color::Color,
    interval::{self, Interval},
    perlin::Perlin,
    rt_image::RtImage,
    vec3::Point3,
};

pub enum Texture {
    Solid(SolidColor),
    Checker(CheckerTexture),
    Image(ImageTexture),
    Noise(NoiseTexture),
}

impl Texture {
    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Texture::Solid(t) => t.value(u, v, p),
            Texture::Checker(t) => t.value(u, v, p),
            Texture::Image(t) => t.value(u, v, p),
            Texture::Noise(t) => t.value(u, v, p),
        }
    }
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Texture {
        Texture::Solid(SolidColor { color_value })
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> Texture {
        Texture::Solid(SolidColor {
            color_value: Color::new(red, green, blue),
        })
    }

    pub fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<Texture>,
    odd: Arc<Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<Texture>, odd: Arc<Texture>) -> Texture {
        Texture::Checker(CheckerTexture {
            inv_scale: 1. / scale,
            even,
            odd,
        })
    }

    pub fn from_color(scale: f64, c1: Color, c2: Color) -> Texture {
        Texture::Checker(CheckerTexture {
            inv_scale: 1. / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        })
    }

    pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RtImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Texture {
        Texture::Image(ImageTexture {
            image: RtImage::new(filename),
        })
    }

    pub fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(0., 1., 1.);
        }

        let u = u.clamp(0., 1.);
        let v = v.clamp(0., 1.);

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;

        self.image.pixel_data(i, j)
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Texture {
        Texture::Noise(NoiseTexture {
            noise: Perlin::new(),
            scale,
        })
    }

    pub fn default() -> Texture {
        NoiseTexture::new(1.)
    }

    pub fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let s = self.scale * *p;
        Color::new(1., 1., 1.) * 0.5 * (1. + (s.z() + 10. * self.noise.turb(&s)).sin())
    }
}

