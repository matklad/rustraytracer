use rand;

use super::Pixel;
use super::utils::ScreenPoint;


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
        let diff = [1.0 / self.resolution[0] as f64, 1.0 / self.resolution[1] as f64];
        (0..self.resolution[0])
            .flat_map(|x| (0..self.resolution[1]).map(move |y| [x as f64, y as f64]))
            .map(|p| {
                let mut square = [0.0, 0.0];
                for i in 0..2 {
                    square[i] = (p[i] + 0.5) * diff[i];
                    if self.jitter {
                        square[i] += rand::random::<f64>() % (diff[i] / 2.0);
                    }
                    assert!(0.0 <= square[i] && square[i] <= 1.0);
                    square[i] = (square[i] - 0.5) * 2.0;
                }
                Sample {
                    pixel: square
                }
            }).collect()
    }
}
