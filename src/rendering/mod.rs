mod image;

use geom::{UnitVector, Dot};
use geom::shape::Intersection;
use color::Color;

use scene::Scene;
use scene::Light;
use scene::Primitive;
use self::image::new_image;

pub use self::image::{Image, Pixel};


type RelPixel = [f64; 2];

trait RelPixelExt {
    fn to_absolute(&self, resolution: Pixel) -> Pixel;
}

impl RelPixelExt for RelPixel {
    fn to_absolute(&self, resolution: Pixel) -> Pixel {
        let mut result = [0, 0];
        for i in 0..2 {
            assert!(-0.5 < self[i] && self[i] < 0.5);
            result[i] = (resolution[i] as f64 * (self[i] + 0.5)) as u32;
        }
        result
    }
}

trait PixelExt {
    fn to_relative(&self, resolution: Pixel) -> RelPixel;
}

impl PixelExt for Pixel {
    fn to_relative(&self, resolution: Pixel) -> RelPixel {
        let mut result = [0.0, 0.0];
        for i in 0..2 {
            let res = resolution[i];
            assert!(self[i] < res);
            let pixel_width = 1.0 / (res as f64);
            result[i] = (((self[i] as f64) + 0.5) * pixel_width) - 0.5;
            assert!(-0.5 < result[i] && result[i] < 0.5);
        }
        result
    }
}


#[derive(Clone, Copy)]
struct Sample {
    screen_point: [f64; 2],
    weight: f64
}

trait Sampler {
    fn sample(&self) -> Vec<Sample>;
}

struct SimpleSampler {
    resolution: Pixel
}

impl SimpleSampler {
    fn new(resolution: Pixel) -> SimpleSampler {
        SimpleSampler { resolution: resolution }
    }
}


impl Sampler for SimpleSampler {
    fn sample(&self) -> Vec<Sample> {
        (0..self.resolution[0])
            .flat_map(|x| (0..self.resolution[1]).map(move |y| [x, y]))
            .map(|p| Sample {
                screen_point: p.to_relative(self.resolution),
                weight: 1.0
            }).collect()
    }
}


type Filter = Fn(Pixel, &Vec<(Sample, Color)>) -> Image;

fn box_filter(resolution: Pixel, samples: &Vec<(Sample, Color)>) -> Image {
    let mut image = new_image(resolution);
    for &(sample, radiance) in samples.iter() {
        let pixel = sample.screen_point.to_absolute(resolution);
        image[pixel] = image[pixel] + radiance * sample.weight;
    }
    image
}

pub struct Renderer<'a> {
    scene: &'a Scene,
    sampler: Box<Sampler>,
    filter: Box<Filter>,
    resolution: Pixel,
}


impl<'a> Renderer<'a> {
    pub fn new(scene: &Scene, resolution: Pixel) -> Renderer {
        Renderer {scene: scene,
                  sampler: Box::new(SimpleSampler::new(resolution)),
                  filter: Box::new(box_filter),
                  resolution: resolution}
    }

    pub fn render(&self) -> Image {
        let samples = self.sampler.sample()
            .into_iter()
            .map(|s| {
                let ray = self.scene.camera.cast_ray(s.screen_point);
                let radiance = match self.scene.find_obstacle(&ray) {
                    Some((obj, point)) => self.colorize(ray.direction, &obj, point),
                    None => self.scene.background_color
                };
                (s, radiance)
            }).collect();

        (self.filter)(self.resolution, &samples)
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

trait ColoredPrimitive {
    fn colorize_ambient(&self, ambient: Color) -> Color;
    fn colorize_diffuse(&self, light: &Light, light_direction: UnitVector,
                        normal: UnitVector) -> Color;
    fn colorize_specular(&self,  view_direction: UnitVector,
                         light: &Light, light_direction: UnitVector,
                         normal: UnitVector) -> Color;
}


impl ColoredPrimitive for Primitive {
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
