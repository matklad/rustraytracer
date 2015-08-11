mod filters;
mod image;

use geom::{UnitVector, Dot};
use geom::shape::Intersection;
use color::Color;

use scene::Scene;
use scene::Light;
use scene::Primitive;
use self::filters::Filter;
use self::image::new_image;

pub use self::image::{Image, Pixel};
pub use self::filters::SmoothingFilter;


pub struct Renderer<'a> {
    scene: &'a Scene,
    pub resolution: Pixel,
    filter: Box<Filter>,
}


impl<'a> Renderer<'a> {
    pub fn new<F: Filter + 'static>(scene: &Scene, resolution: Pixel, filter: F) -> Renderer {
        let resolution = filter.process_resolution(resolution);
        Renderer {scene: scene,
                  filter: Box::new(filter),
                  resolution: resolution}
    }

    pub fn render(&self) -> Image {
        let image = new_image(self.resolution, |x, y| {
            let ray = self.scene.camera.cast_ray(self.resolution, [x, y]);
            match self.scene.find_obstacle(&ray) {
                Some((obj, point)) => self.colorize(ray.direction, &obj, point),
                None => self.scene.background_color
            }
        });

        self.filter.process_image(image)
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
