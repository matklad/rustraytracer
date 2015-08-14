mod samplers;
mod utils;
mod filters;
mod config;

use geom::{UnitVector, Dot};
use geom::shape::Intersection;
use color::Color;
use datastructures::Matrix;

use scene::Scene;
use scene::Light;
use scene::Primitive;
use self::samplers::{Sampler, StratifiedSampler};
use self::filters::{Filter, box_filter};

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
        Renderer {scene: scene,
                  sampler: Box::new(StratifiedSampler::new(
                      [config.resolution[0], config.resolution[1]], true)),
                  filter: Box::new(box_filter(config.filter.extent, config.resolution)),
                  resolution: config.resolution}
    }

    pub fn render(&self) -> Image {
        let samples = self.sampler.sample()
            .into_iter()
            .map(|s| {
                let ray = self.scene.camera.cast_ray(s.pixel);
                let radiance = match self.scene.find_obstacle(&ray) {
                    Some((obj, point)) => self.colorize(ray.direction, &obj, point),
                    None => self.scene.background_color
                };
                (s, radiance)
            }).collect();

        self.filter.apply(self.resolution, &samples)
    }

    fn colorize(&self, view_direction: UnitVector,
                primitive: &Primitive, intersection: Intersection) -> Color {
        let mut result = primitive.colorize_ambient(self.scene.ambient_light);
        let visible_lights = self.scene.lights.iter()
            .filter(|&light| self.scene.is_visible(light.position(), intersection.point));

        for light in visible_lights {
            let light_direction = light.position().direction_to(intersection.point);
            let normal = intersection.normal;
            result = result
                + primitive.colorize_diffuse(&light, light_direction, normal)
                + primitive.colorize_specular(view_direction, &light,
                                           light_direction, normal);
        }
        result
    }
}

trait PrimitiveExt {
    fn colorize_ambient(&self, ambient: Color) -> Color;
    fn colorize_diffuse(&self, light: &Light, light_direction: UnitVector,
                        normal: UnitVector) -> Color;
    fn colorize_specular(&self,  view_direction: UnitVector,
                         light: &Light, light_direction: UnitVector,
                         normal: UnitVector) -> Color;
}


impl PrimitiveExt for Primitive {
    fn colorize_ambient(&self, ambient: Color) -> Color {
        self.material.color * ambient
    }

    fn colorize_diffuse(&self, light: &Light, light_direction: UnitVector,
                        normal: UnitVector) -> Color {
        let k = (-light_direction.dot(normal)).max(0.0) * self.material.diffuse;
        self.material.color * light.color() * k
    }

    fn colorize_specular(&self,  view_direction: UnitVector,
                         light: &Light, light_direction: UnitVector,
                         normal: UnitVector) -> Color {

        let r = light_direction.reflect(normal);
        let k = (-r.dot(view_direction)).max(0.0).powf(self.material.specular);
        light.color() * k
    }
}
