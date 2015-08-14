use color::Color;
use datastructures::Matrix;
use scene::ScreenPoint;
use super::{Image, Pixel};
use super::samplers::Sample;
use super::utils::{to_scren_point, from_uniform};


pub struct Filter {
    extent: ScreenPoint,
    resolution: Pixel,
    weight: fn(f64, f64) -> f64
}

impl Filter {
    pub fn apply(&self, resolution: Pixel, samples: &Vec<(Sample, Color)>) -> Image {
        let mut image = Image::fill(resolution, Color::new(0.0, 0.0, 0.0));
        let mut weights = Matrix::<f64>::fill(resolution, 0.0);
        for &(sample, radiance) in samples.iter() {
            for pixel in self.neighbours(sample.pixel) {
                let diff = to_scren_point(self.resolution, pixel);
                let weight = (self.weight)(diff.x.abs(), diff.y.abs());
                image[pixel] = image[pixel] + radiance * weight;
                weights[pixel] += weight;
            }
        }

        for (i, weight) in weights.iter() {
            if weight != 0.0 {
                image[i] = image[i] / weight;
            }

        }

        image
    }

    fn neighbours(&self, point: ScreenPoint) -> Vec<Pixel> {
        let discretize_range = |lower: f64, upper: f64| {
            assert!(lower <= upper);
            lower.ceil() as i32 .. upper.floor() as i32 + 1
        };

        let ok_pixel = |x, y| {
            0 <= x && x < self.resolution[0] as i32 &&
                0 <= y && y < self.resolution[1] as i32
        };

        let point = from_uniform(self.resolution, point);
        let lower = point - self.extent;
        let upper = point + self.extent;
        let mut result = Vec::new();
        for x in discretize_range(lower.x, upper.x) {
            for y in discretize_range(lower.y, upper.y) {
                if ok_pixel(x, y) {
                    result.push([x as u32, y as u32]);
                }
            }
        }
        result
    }
}


fn box_weight(_x: f64, _y: f64) -> f64 {
    1.0
}

pub fn box_filter(extent: [f64; 2], resolution: Pixel) -> Filter {
    Filter { extent: ScreenPoint::from(extent),
             resolution: resolution,
             weight: box_weight
    }
}
