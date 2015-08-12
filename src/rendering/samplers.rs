use super::image::Pixel;
use super::utils::{PixelExt, RelPixel};


#[derive(Clone, Copy)]
pub struct Sample {
    pub pixel: RelPixel,
}


pub trait Sampler {
    fn sample(&self) -> Vec<Sample>;
}

pub struct SimpleSampler {
    resolution: Pixel
}

impl SimpleSampler {
    pub fn new(resolution: Pixel) -> SimpleSampler {
        SimpleSampler { resolution: resolution }
    }
}


impl Sampler for SimpleSampler {
    fn sample(&self) -> Vec<Sample> {
        (0..self.resolution[0])
            .flat_map(|x| (0..self.resolution[1]).map(move |y| [x, y]))
            .map(|p| Sample {
                pixel: p.to_relative(self.resolution)
            }).collect()
    }
}
