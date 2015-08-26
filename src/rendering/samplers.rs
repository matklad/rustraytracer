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
    range: [Range<u32>; 2],
    jitter: bool
}

impl StratifiedSampler {
    pub fn new(resolution: Pixel, config: SamplerConfig) -> StratifiedSampler {
        match config {
            SamplerConfig::Stratified { samples_per_pixel, jitter} => {
                let width = resolution[0] * samples_per_pixel;
                let height = resolution[1] * samples_per_pixel;
                StratifiedSampler {
                    resolution: [width, height],
                    range: [0..width, 0..height],
                    jitter: jitter
                }
            }
        }
    }

    fn width(&self) -> u32 {
        self.range[0].end - self.range[0].start
    }

    fn height(&self) -> u32 {
        self.range[1].end - self.range[1].start
    }

    fn area(&self) -> u32 {
        self.width() * self.height()
    }
}


impl Sampler for StratifiedSampler {
    fn sample(&self) -> Vec<Sample> {
        let mut result = Vec::with_capacity(self.area() as usize);
        for x in self.range[0].clone() {
            for y in self.range[1].clone() {
                let jitter = if self.jitter {
                    ScreenPoint::new(
                        rand::random::<f64>() % 0.5,
                        rand::random::<f64>() % 0.5)
                } else {
                    ScreenPoint::new(0.0, 0.0)
                };

                result.push(Sample {
                    pixel: to_uniform(self.resolution, ScreenPoint::from([x, y]) + jitter)
                })
            }
        }
        result
    }

    fn split(&self, n_parts: u16) -> Vec<Box<Sampler>> {
        let square_width = (self.area() as f64 / (n_parts as f64)).sqrt() as u32;
        let mut result = Vec::new();
        let mx = self.width() / square_width;
        let my = self.height() / square_width;
        for rx in partition(&self.range[0], mx) {
            for ry in partition(&self.range[1], my) {
                result.push(Box::new(StratifiedSampler {
                    resolution: self.resolution,
                    range: [rx.clone(), ry.clone()],
                    jitter: self.jitter}) as Box<Sampler>)
            }
        }
        result
    }
}


fn partition(r: &Range<u32>, n: u32) -> Vec<Range<u32>> {
    let mut result = Vec::new();
    let step = (r.end - r.start) / n;
    let mut hit_end = false;
    for i in 0..n {
        let start = r.start + step * i;
        let end = if i != n - 1 { r.start + step * (i + 1) } else { r.end };
        assert!(start <= end);
        assert!(r.start <= start);
        assert!(end <= r.end);
        if end == r.end {
            hit_end = true;
        }
        result.push(start..end)
    }
    assert!(hit_end);
    result
}
