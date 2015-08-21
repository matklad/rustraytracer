mod samplers;
mod utils;
mod filters;
mod config;

use std::fmt;

use color::Color;
use datastructures::Matrix;
use geom::{UnitVector, Dot};
use scene::{Intersection, Scene, Texture};
use utils::time_it;
use self::filters::Filter;
use self::samplers::{Sampler, StratifiedSampler};

pub use self::config::TracerConfig;


pub type Pixel = [u32; 2];

pub type Image = Matrix<Color>;


pub struct TracingStats {
    pub rendering_time: f64,
    pub filtering_time: f64
}

impl fmt::Display for TracingStats {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "Rendering:   {:.2}s\nFiltering:   {:.2}s",
               self.rendering_time, self.filtering_time)
    }

}

pub struct Tracer<'a> {
    scene: &'a Scene,
    sampler: Box<Sampler>,
    filter: Box<Filter>,
}



impl<'a> Tracer<'a> {
    pub fn new(scene: &Scene, config: TracerConfig) -> Tracer {
        Tracer {
            scene: scene,
            sampler: Box::new(StratifiedSampler::new(config.resolution, config.sampler)),
            filter: Box::new(Filter::new(config.resolution, config.filter))
        }
    }

    pub fn render(&self) -> (Image, TracingStats) {

        let (samples, rendering_time) = time_it(|| {
            self.sampler.sample()
            .into_iter()
            .map(|s| {
                let ray = self.scene.camera.cast_ray(s.pixel);
                let radiance = match self.scene.find_obstacle(&ray) {
                    Some(ref intersection) => self.colorize(ray.direction, intersection),
                    None => self.scene.background_color
                };
                (s, radiance)
            }).collect()
        });

        let (image, filtering_time) = time_it(|| self.filter.apply(&samples));
        (image, TracingStats {rendering_time: rendering_time,
                              filtering_time: filtering_time})
    }

    fn colorize(&self, view_direction: UnitVector, intersection: &Intersection) -> Color {
        let mut result = intersection.colorize_ambient(self.scene.ambient_light);
        let visible_lights = self.scene.lights.iter()
            .filter(|&light| self.scene.is_visible(light.position(), &intersection));

        for light in visible_lights {
            let light_direction = light.position().direction_to(intersection.geom.point);
            let illumination = light.illuminate(intersection.geom.point);
            result = result
                + intersection.colorize_diffuse(illumination, light_direction)
                + intersection.colorize_specular(illumination, light_direction, view_direction);
        }
        result
    }
}

trait IntersectionExt {
    fn colorize_ambient(&self, illumination: Color) -> Color;

    fn colorize_diffuse(&self,
                        illumination: Color,
                        light_direction: UnitVector)
                        -> Color;

    fn colorize_specular(&self,
                         illumination: Color,
                         light_direction: UnitVector,
                         view_direction: UnitVector)
                         -> Color;
}


impl<'a> IntersectionExt for Intersection<'a> {
    fn colorize_ambient(&self, illumination: Color) -> Color {
        self.primitive.material.color.at(&self.geom) * illumination
    }

    fn colorize_diffuse(&self,
                        illumination: Color,
                        light_direction: UnitVector)
                        -> Color {

        let k = (-light_direction.dot(self.geom.normal)).max(0.0) * self.primitive.material.diffuse;
        self.primitive.material.color.at(&self.geom) * illumination * k
    }

    fn colorize_specular(&self,
                         illumination: Color,
                         light_direction: UnitVector,
                         view_direction: UnitVector)
                         -> Color {

        let r = light_direction.reflect(self.geom.normal);
        let k = (-r.dot(view_direction)).max(0.0).powf(self.primitive.material.specular);
        illumination * k
    }
}
