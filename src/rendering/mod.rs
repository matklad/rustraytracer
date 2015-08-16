mod samplers;
mod utils;
mod filters;
mod config;

use color::Color;
use datastructures::Matrix;
use geom::{UnitVector, Dot};
use scene::{Intersection, Light, Scene, Texture};
use self::filters::Filter;
use self::samplers::{Sampler, StratifiedSampler};

pub use self::config::TracerConfig;


pub type Pixel = [u32; 2];

pub type Image = Matrix<Color>;


pub struct Tracer<'a> {
    scene: &'a Scene,
    sampler: Box<Sampler>,
    filter: Box<Filter>,
    resolution: Pixel,
}


impl<'a> Tracer<'a> {
    pub fn new(scene: &Scene, config: TracerConfig) -> Tracer {
        Tracer {
            scene: scene,
            sampler: Box::new(StratifiedSampler::new(config.resolution, config.sampler)),
            filter: Box::new(Filter::new(config.resolution, config.filter)),
            resolution: config.resolution
        }
    }

    pub fn render(&self) -> Image {
        let samples = self.sampler.sample()
            .into_iter()
            .map(|s| {
                let ray = self.scene.camera.cast_ray(s.pixel);
                let radiance = match self.scene.find_obstacle(&ray) {
                    Some(ref intersection) => self.colorize(ray.direction, intersection),
                    None => self.scene.background_color
                };
                (s, radiance)
            }).collect();

        self.filter.apply(self.resolution, &samples)
    }

    fn colorize(&self, view_direction: UnitVector, intersection: &Intersection) -> Color {
        let mut result = intersection.colorize_ambient(self.scene.ambient_light);
        let visible_lights = self.scene.lights.iter()
            .filter(|&light| self.scene.is_visible(light.position(), &intersection));

        for light in visible_lights {
            let light_direction = light.position().direction_to(intersection.geom.point);
            result = result
                + intersection.colorize_diffuse(&light, light_direction)
                + intersection.colorize_specular(view_direction, &light, light_direction);
        }
        result
    }
}

trait IntersectionExt {
    fn colorize_ambient(&self, ambient: Color) -> Color;

    fn colorize_diffuse(&self,
                        light: &Light,
                        light_direction: UnitVector) -> Color;

    fn colorize_specular(&self,
                         view_direction: UnitVector,
                         light: &Light,
                         light_direction: UnitVector) -> Color;
}


impl<'a> IntersectionExt for Intersection<'a> {
    fn colorize_ambient(&self, ambient: Color) -> Color {
        self.primitive.material.color.at(&self.geom) * ambient
    }

    fn colorize_diffuse(&self,
                        light: &Light,
                        light_direction: UnitVector) -> Color {

        let k = (-light_direction.dot(self.geom.normal)).max(0.0) * self.primitive.material.diffuse;
        self.primitive.material.color.at(&self.geom) * light.color() * k
    }

    fn colorize_specular(&self,
                         view_direction: UnitVector,
                         light: &Light,
                         light_direction: UnitVector) -> Color {

        let r = light_direction.reflect(self.geom.normal);
        let k = (-r.dot(view_direction)).max(0.0).powf(self.primitive.material.specular);
        light.color() * k
    }
}
