use crate::vec3::Color;
use crate::vec3::Point3;

use crate::perlin::Perlin;

use std::rc::Rc;
use std::cell::RefCell;

use image::io::Reader;
use image::ImageBuffer;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.color_value
    }
}

impl SolidColor {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        SolidColor {
            color_value: Color(red, green, blue),
        }
    }
}

pub struct CheckerTexture {
    pub odd: Rc<RefCell<dyn Texture>>,
    pub even: Rc<RefCell<dyn Texture>>
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0*p.x()).sin()*(10.0*p.y()).sin()*(10.0*p.z()).sin();
        if sines < 0.0 {
            return self.odd.borrow().value(u, v, p);
        } else {
            return self.even.borrow().value(u, v, p);
        }
    }
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> Self {
        CheckerTexture {
            even: Rc::new(RefCell::new(SolidColor{color_value: c1})),
            odd: Rc::new(RefCell::new(SolidColor{color_value: c2}))
        }
    }
}

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale*p.z() + 10.0*self.noise.turb(p)).sin())
    }
}

pub struct ImageTexture {
    data: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        

        Color(0.0, 0.0, 0.0)
    }
}

impl ImageTexture {
    fn new(filename: String) -> Self {
        let bytes_per_pixel: u32 = 3;
        let limage = Reader::open(filename).unwrap().decode().unwrap().as_rgb8().clone();

        let (width, height) = limage.unwrap().dimensions();

        // if (!data) {
        //     std::cerr << "ERROR: Could not load texture image file '" << filename << "'.\n";
        //     width = height = 0;
        // }

        let bytes_per_scanline = bytes_per_pixel*width;
        
        ImageTexture {
            data: limage,
            width: width,
            height: height,
            bytes_per_scanline: bytes_per_scanline
        }
    }
}