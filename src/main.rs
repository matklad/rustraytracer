extern crate rustraytracer;

use std::fs;
use std::io;

use rustraytracer::color::{palette, Color};
use rustraytracer::display::{PpmWriter, ImageDisplay};
use rustraytracer::geom::shape::{Mesh, Triangle, Sphere};
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
            ambient_light: Color::from("#555"),
            background: palette::BLUE,
        },
        SmoothingFilter(2)
    );

    let t1 = Triangle::new(p(0.0, -3.0, 0.0),
                           p(-3.0, 0.0, 0.0),
                           p(0.0, 0.0, 3.0));

    let t2 = Triangle::new(p(-3.0, 0.0, 0.0),
                           p(0.0, 3.0, 0.0),
                           p(0.0, 0.0, 3.0));

    let mesh = Mesh::new(vec![
        t1,
        t2
            ]);

    let obj = Object::new(
        // Sphere::new(p(0.0, 0.0, 0.0), 1.0),
        mesh,
        Color::from("#FFF")
    );

    scene.add_object(obj);
    let light_pos = p(6.0, 1.0, 0.0);
    // scene.add_object(Object::new(
    //     Sphere::new(light_pos, 0.3),
    //     Color::from("#FF0")
    //     ));
    scene.add_light(Light::new(Color::from("#F00"),
                               light_pos));

    let image = scene.render();
    let path = "./out.ppm";
    let mut file = io::BufWriter::new(fs::File::create(path).unwrap());
    let mut display = PpmWriter::new(&mut file);
    display.draw(&image).unwrap();
}
