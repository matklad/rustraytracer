extern crate rustraytracer;

use std::fs;
use std::io;

use rustraytracer::color::Color;
use rustraytracer::display::{PpmWriter, ImageDisplay};
use rustraytracer::geom::shape::{Mesh};
use rustraytracer::geom::shortcuts::{p, v};
use rustraytracer::scene::{Scene, SceneConfig, CameraConfig, Light, SmoothingFilter, Object};


#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut scene = Scene::new(
        SceneConfig {
            camera: CameraConfig {
                position: p(0.0, 40.0, 90.0),
                focus_distance: 80.0,
                up: v(0.0, 0.0, -1.0).direction(),
                resolution: [320, 240],
                size: [40.0, 30.0],
                ..Default::default()
            },
            ambient_light: Color::from("#444"),
            background: Color::from("#115"),
        },
        SmoothingFilter(1)
    );


    let mut teapot = io::BufReader::new(fs::File::open("./utah.obj").unwrap());
    let mesh = Mesh::from_obj(&mut teapot).unwrap();

    let obj = Object::new(mesh, Color::from("#FFF"));
    scene.add_object(obj);
    scene.add_light(Light::new(
        Color::from("#FFF"),
        p(80.0, 80.0, 50.0)));

    let image = scene.render();
    let path = "./out.ppm";
    let mut file = io::BufWriter::new(fs::File::create(path).unwrap());
    let mut display = PpmWriter::new(&mut file);
    display.draw(&image).unwrap();
}
