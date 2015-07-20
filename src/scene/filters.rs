use super::{SceneConfig, Image};
use color::Color;
use super::camera::CameraConfig;
use super::image::new_image;

pub trait Filter {
    fn process_config(&self, config: SceneConfig) -> SceneConfig {
        config
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
    fn process_config(&self, config: SceneConfig) -> SceneConfig {
        let resolution = [config.camera.resolution[0] * self.x(),
                          config.camera.resolution[1] * self.x()];
        SceneConfig {
            camera: CameraConfig {
                resolution: resolution,
                ..config.camera
            },
            ..config
        }
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
