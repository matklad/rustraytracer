use color::Color;
use datastructures::Matrix;
use super::utils::{RelPixelExt};
use super::image::{Image, Pixel};
use super::samplers::Sample;


pub type Filter = Fn(Pixel, &Vec<(Sample, Color)>) -> Image;

pub fn box_filter(resolution: Pixel, samples: &Vec<(Sample, Color)>) -> Image {
    let mut image = Image::fill(resolution, Color::new(0.0, 0.0, 0.0));
    let mut weights = Matrix::<u32>::fill(resolution, 0);
    for &(sample, radiance) in samples.iter() {
        let pixel = sample.pixel.to_absolute(resolution);
        image[pixel] = image[pixel] + radiance;
        weights[pixel] += 1;
    }

    for (i, cnt) in weights.iter() {
        image[i] = image[i] / (cnt as f64);
    }

    image
}
