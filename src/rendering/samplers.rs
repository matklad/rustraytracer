use std::ops::Range;

use rand;

use scene::ScreenPoint;
use super::Pixel;
use super::utils::to_uniform;
use super::config::SamplerConfig;


#[derive(Clone, Copy)]
pub struct Sample {
    pub pixel: ScreenPoint,
}


pub trait Sampler: Send + Sync {
    fn sample(&self) -> Vec<Sample>;
    fn split(&self, n_parts: u16) -> Vec<Box<Sampler>>;
}

pub struct StratifiedSampler {
    resolution: Pixel,
    range: Range<u32>,
    jitter: bool
}

impl StratifiedSampler {
    pub fn new(resolution: Pixel, config: SamplerConfig) -> StratifiedSampler {
        match config {
            SamplerConfig::Stratified { samples_per_pixel, jitter} =>
                StratifiedSampler {
                    resolution: [resolution[0] * samples_per_pixel,
                                 resolution[1] * samples_per_pixel],
                    range: 0..(resolution[1] * samples_per_pixel),
                    jitter: jitter
                }
        }
    }
}


impl Sampler for StratifiedSampler {
    fn sample(&self) -> Vec<Sample> {
        self.range.clone()
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

    fn split(&self, n_parts: u16) -> Vec<Box<Sampler>> {
        let step_size = self.resolution[0] / n_parts as u32;
        (0..n_parts).map(|i| {
            let start = (i as u32) * step_size;
            let stop = if i == n_parts - 1 {
                self.resolution[0]
            } else {
                start + step_size
            };

            Box::new(StratifiedSampler {
                resolution: self.resolution,
                range: (start..stop),
                jitter: self.jitter}) as Box<Sampler>
        }).collect()
    }
}
