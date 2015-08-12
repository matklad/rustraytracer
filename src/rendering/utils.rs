use super::image::Pixel;

pub type RelPixel = [f64; 2];

pub trait RelPixelExt {
    fn to_absolute(&self, resolution: Pixel) -> Pixel;
}

impl RelPixelExt for RelPixel {
    fn to_absolute(&self, resolution: Pixel) -> Pixel {
        let mut result = [0, 0];
        for i in 0..2 {
            assert!(-0.5 < self[i] && self[i] < 0.5);
            result[i] = (resolution[i] as f64 * (self[i] + 0.5)) as u32;
        }
        result
    }
}

pub trait PixelExt {
    fn to_relative(&self, resolution: Pixel) -> RelPixel;
}

impl PixelExt for Pixel {
    fn to_relative(&self, resolution: Pixel) -> RelPixel {
        let mut result = [0.0, 0.0];
        for i in 0..2 {
            let res = resolution[i];
            assert!(self[i] < res);
            let pixel_width = 1.0 / (res as f64);
            result[i] = (((self[i] as f64) + 0.5) * pixel_width) - 0.5;
            assert!(-0.5 < result[i] && result[i] < 0.5);
        }
        result
    }
}
