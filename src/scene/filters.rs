use color::Color;
use super::Image;
use super::image::{new_image, Pixel};

pub trait Filter {
    fn process_resolution(&self, r: Pixel) -> Pixel {
        r
    }

    fn process_image(&self, image: Image) -> Image {
        image
    }
}

pub struct NopFilter;
impl Filter for NopFilter {}

pub struct SmoothingFilter(pub u32);

impl SmoothingFilter {
    fn x(&self) -> u32 {
        self.0 as u32
    }
}

impl Filter for SmoothingFilter {
    fn process_resolution(&self, r: Pixel) -> Pixel {
        [r[0] * self.x(), r[1] * self.x()]
    }

    fn process_image(&self, image: Image) -> Image {
        let resolution = [image.width() / self.x(),
                          image.height() / self.x()];
        new_image(resolution, |x, y| {
            let x = x * self.x();
            let y = y * self.x();
            (x..x +  self.x())
                .flat_map(|x| (y..y + self.x()).map(move |y| (x, y)))
                .map(|(x, y)| image.at(x, y))
                .fold(Color::new(0.0, 0.0, 0.0), |acc, item| acc + item)
                / ((self.x() * self.x()) as f64)
        })
    }
}
