extern crate rustraytracer;

use std::fs;
use std::io;

use rustraytracer::color::{palette, Color};
use rustraytracer::display::{PpmWriter, ImageDisplay};
use rustraytracer::geom::shape::{Triangle};
use rustraytracer::geom::shortcuts::p;
use rustraytracer::scene::{Scene, SceneConfig, CameraConfig, Light, SmoothingFilter, Object};


#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut scene = Scene::new(
        SceneConfig {
            camera: CameraConfig {
                resolution: [640, 480],
                ..Default::default()
            },
            ambient_light: Color::from("#111"),
            background: palette::BLUE,
        },
        SmoothingFilter(2)
    );
    let sphere = Object::new(
        // Sphere::new(p(0.0, 0.0, 0.0), 1.0),
        Triangle::new(p(0.0, -2.0, -1.0),
                      p(0.0, 2.0, -1.0),
                      p(0.0, 0.0, 1.0)),
        Color::from("#FFF")
    );

    scene.add_object(sphere);
    scene.add_light(Light::new(Color::from("#FF0"),
                               p(6.0, 5.0, 5.0)));

    let image = scene.render();
    let path = "../out.ppm";
    let mut file = io::BufWriter::new(fs::File::create(path).unwrap());
    let mut display = PpmWriter::new(&mut file);
    display.draw(&image).unwrap();
}
