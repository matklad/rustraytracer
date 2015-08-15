mod samplers;
mod utils;
mod filters;
mod config;

use color::Color;
use datastructures::Matrix;
use geom::{UnitVector, Dot};
use scene::{Intersection, Light, Primitive, Scene};
use self::filters::Filter;
use self::samplers::{Sampler, StratifiedSampler};

pub use self::config::RendererConfig;


pub type Pixel = [u32; 2];

pub type Image = Matrix<Color>;


pub struct Renderer<'a> {
    scene: &'a Scene,
    sampler: Box<Sampler>,
    filter: Box<Filter>,
    resolution: Pixel,
}


impl<'a> Renderer<'a> {
    pub fn new(scene: &Scene, config: RendererConfig) -> Renderer {
        Renderer {
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
        let mut result = intersection.primitive.colorize_ambient(self.scene.ambient_light);
        let visible_lights = self.scene.lights.iter()
            .filter(|&light| self.scene.is_visible(light.position(), &intersection));

        for light in visible_lights {
            let light_direction = light.position().direction_to(intersection.geom.point);
            let normal = intersection.geom.normal;
            result = result
                + intersection.primitive.colorize_diffuse(&light, light_direction, normal)
                + intersection.primitive.colorize_specular(view_direction, &light,
                                                           light_direction, normal);
        }
        result
    }
}

trait PrimitiveExt {
    fn colorize_ambient(&self, ambient: Color) -> Color;

    fn colorize_diffuse(&self,
                        light: &Light,
                        light_direction: UnitVector,
                        normal: UnitVector) -> Color;

    fn colorize_specular(&self,
                         view_direction: UnitVector,
                         light: &Light,
                         light_direction: UnitVector,
                         normal: UnitVector) -> Color;
}


impl PrimitiveExt for Primitive {
    fn colorize_ambient(&self, ambient: Color) -> Color {
        self.material.color * ambient
    }

    fn colorize_diffuse(&self,
                        light: &Light,
                        light_direction: UnitVector,
                        normal: UnitVector) -> Color {

        let k = (-light_direction.dot(normal)).max(0.0) * self.material.diffuse;
        self.material.color * light.color() * k
    }

    fn colorize_specular(&self,
                         view_direction: UnitVector,
                         light: &Light,
                         light_direction: UnitVector,
                         normal: UnitVector) -> Color {

        let r = light_direction.reflect(normal);
        let k = (-r.dot(view_direction)).max(0.0).powf(self.material.specular);
        light.color() * k
    }
}
