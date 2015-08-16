use rand;

use scene::ScreenPoint;
use super::Pixel;
use super::utils::to_uniform;
use super::config::SamplerConfig;


#[derive(Clone, Copy)]
pub struct Sample {
    pub pixel: ScreenPoint,
}


pub trait Sampler {
    fn sample(&self) -> Vec<Sample>;
}

pub struct StratifiedSampler {
    resolution: Pixel,
    jitter: bool
}

impl StratifiedSampler {
    pub fn new(resolution: Pixel, config: SamplerConfig) -> StratifiedSampler {
        match config {
            SamplerConfig::Stratified { samples_per_pixel, jitter} =>
                StratifiedSampler {
                    resolution: [resolution[0] * samples_per_pixel,
                                 resolution[1] * samples_per_pixel],
                    jitter: jitter
                }
        }
    }
}


impl Sampler for StratifiedSampler {
    fn sample(&self) -> Vec<Sample> {
        (0..self.resolution[0])
            .flat_map(|x| (0..self.resolution[1]).map(move |y| [x , y]))
            .map(|p| {
                let jitter = if self.jitter {
                    ScreenPoint::new(
                        rand::random::<f64>() % 0.5,
                        rand::random::<f64>() % 0.5)
                } else {
                    ScreenPoint::new(0.0, 0.0)
                };

                Sample {
                    pixel: to_uniform(self.resolution, ScreenPoint::from(p) + jitter)
                }
            }).collect()
    }
}
