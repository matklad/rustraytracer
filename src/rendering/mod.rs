mod samplers;
mod utils;
mod filters;
mod config;

use std::{fmt, thread};
use std::sync::Arc;

use color::Color;
use datastructures::Matrix;
use geom::{UnitVector, Dot};
use geom::ray::Ray;
use scene::{Intersection, Scene, Texture};
use utils::time_it;
use self::filters::Filter;
use self::samplers::{Sample, Sampler, StratifiedSampler};

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

pub struct Tracer {
    scene: Scene,
    sampler: Box<Sampler>,
    filter: Box<Filter>,
    n_reflections: u32,
    n_threads: u16,
}



impl Tracer {
    pub fn new(scene: Scene, config: TracerConfig) -> Tracer {
        Tracer {
            scene: scene,
            sampler: Box::new(StratifiedSampler::new(config.resolution, config.sampler)),
            filter: Box::new(Filter::new(config.resolution, config.filter)),
            n_reflections: config.n_reflections,
            n_threads: config.n_threads,
        }
    }

    pub fn render(self) -> (Image, TracingStats) {

        let tracer = Arc::new(self);
        let (results, rendering_time) = time_it(|| {
            let samples = tracer.sampler.sample();
            let chunk_size = samples.len() / tracer.n_threads as usize;
            let chunks: Vec<_> = samples
                .chunks(chunk_size)
                .map(|c| c.to_vec())
                .collect();

            let threads: Vec<_> = chunks.into_iter()
                .map(|chunk| {
                    let tracer = tracer.clone();
                    thread::spawn(move || {
                        tracer.render_samples(&chunk)
                    })
                }).collect();

            let mut results = Vec::new();
            for t in threads {
                results.extend(t.join().unwrap().into_iter());
            }
            results
        });

        let (image, filtering_time) = time_it(|| tracer.filter.apply(&results));
        (image, TracingStats {rendering_time: rendering_time,
                              filtering_time: filtering_time})
    }

    fn render_samples(&self, samples: &[Sample]) -> Vec<(Sample, Color)> {
        samples.into_iter()
            .map(|&s| {
                let ray = self.scene.camera.cast_ray(s.pixel);
                (s, self.radiace(&ray, 0))
            }).collect()
    }

    pub fn radiace(&self, ray: &Ray, level: u32) -> Color {
        match self.scene.find_obstacle(ray) {
            Some(ref intersection) => {
                let arrived_light = self.colorize(ray.direction, intersection);
                let reflectance = intersection.primitive.material.reflectance;
                let has_reflection = level < self.n_reflections
                    && reflectance > 0.0;
                let reflected_light = if has_reflection  {
                    let refl_dir = ray.direction.reflect(intersection.geom.normal);
                    let reflected_ray = self.scene.ray_from(intersection, refl_dir);
                    self.radiace(&reflected_ray, level + 1) * reflectance
                } else {
                    Color::new(0.0, 0.0, 0.0)
                };

                arrived_light + reflected_light
            },
            None => self.scene.background_color
        }
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
