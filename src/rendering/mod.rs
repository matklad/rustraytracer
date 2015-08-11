mod image;

use geom::{UnitVector, Dot};
use geom::shape::Intersection;
use color::Color;

use scene::Scene;
use scene::Light;
use scene::Primitive;
use self::image::new_image;

pub use self::image::{Image, Pixel};


struct Sample {
    screen_point: [f64; 2],
    weight: f64
}

trait Sampler {
    fn sample(&self) -> Vec<Sample>;
}

fn pixel_to_relative(resolution: Pixel, pixel: Pixel) -> Sample {
    let mut screen_point = [0.0, 0.0];
    for i in 0..2 {
        let res = resolution[i];
        assert!(pixel[i] < res);
        let pixel_width = 1.0 / (res as f64);
        screen_point[i] = (((pixel[i] as f64) + 0.5) * pixel_width) - 0.5;
        assert!(-0.5 < screen_point[i] && screen_point[i] < 0.5);
    }
    Sample { screen_point: screen_point, weight: 1.0 }
}

fn relative_to_pixel(resolution: Pixel, relative: [f64; 2]) -> Pixel {
    let mut result = [0, 0];
    for i in 0..2 {
        assert!(-0.5 < relative[i] && relative[i] < 0.5);
        result[i] = (resolution[i] as f64 * (relative[i] + 0.5)) as u32;
    }
    result
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
            .map(|p| pixel_to_relative(self.resolution, p))
            .collect()
    }
}


pub struct Renderer<'a> {
    scene: &'a Scene,
    sampler: Box<Sampler>,
    resolution: Pixel
}


impl<'a> Renderer<'a> {
    pub fn new(scene: &Scene, resolution: Pixel) -> Renderer {
        Renderer {scene: scene,
                  sampler: Box::new(SimpleSampler::new(resolution)),
                  resolution: resolution}
    }

    pub fn render(&self) -> Image {
        let samples = self.sampler.sample();
        let mut image = new_image(self.resolution);
        for s in samples {
            let ray = self.scene.camera.cast_ray(s.screen_point);
            let radiance = match self.scene.find_obstacle(&ray) {
                Some((obj, point)) => self.colorize(ray.direction, &obj, point),
                None => self.scene.background_color
            };
            let pixel = relative_to_pixel(self.resolution, s.screen_point);
            image[pixel] = image[pixel] + radiance * s.weight;
        }

        image
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
