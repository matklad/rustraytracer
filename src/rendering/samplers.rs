use rand;

use scene::ScreenPoint;
use super::Pixel;
use super::utils::to_uniform;


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
    pub fn new(resolution: Pixel, jitter: bool) -> StratifiedSampler {
        StratifiedSampler {
            resolution: resolution,
            jitter: jitter
        }
    }
}


impl Sampler for StratifiedSampler {
    fn sample(&self) -> Vec<Sample> {
        (0..self.resolution[0])
            .flat_map(|x| (0..self.resolution[1]).map(move |y| [x , y]))
            .map(|p| {
                assert!(p[1] < 480);
                let jitter = if self.jitter {
                    ScreenPoint::new(
                        rand::random::<f64>() % 0.5,
                        rand::random::<f64>() % 0.5)
                } else {
                    ScreenPoint::new(0.0, 0.0)
                };

                let pixel = to_uniform(self.resolution, ScreenPoint::from(p) + jitter);
                Sample {
                    pixel: pixel
                }
            }).collect()
    }
}
