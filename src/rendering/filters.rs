use color::Color;
use datastructures::Matrix;
use super::utils::{ScreenPointExt, PixelExt};
use super::{Image, Pixel};
use super::samplers::Sample;


pub struct Filter {
    extent: [f64; 2],
    weight: fn(f64, f64) -> f64
}

impl Filter {
    pub fn apply(&self, resolution: Pixel, samples: &Vec<(Sample, Color)>) -> Image {
        let mut image = Image::fill(resolution, Color::new(0.0, 0.0, 0.0));
        let mut weights = Matrix::<f64>::fill(resolution, 0.0);
        for &(sample, radiance) in samples.iter() {
            // for pixel in sample.pixel.neighbours(resolution, self.extent) {
            let pixel = sample.pixel.to_absolute(resolution);
                let rp = pixel.to_relative(resolution);
                let dx = (rp[0] - sample.pixel[0]).abs();
                let dy = (rp[1] - sample.pixel[1]).abs();
                let weight = (self.weight)(dx, dy);
                image[pixel] = image[pixel] + radiance * weight;
                weights[pixel] += weight;
            // }

        }

        for (i, weight) in weights.iter() {
            if weight != 0.0 {
                image[i] = image[i] / weight;
            }

        }

        image
    }
}


fn box_weight(_x: f64, _y: f64) -> f64 {
    1.0
}

pub fn box_filter(extent: [f64; 2]) -> Filter {

    Filter { extent: extent, weight: box_weight}
}
