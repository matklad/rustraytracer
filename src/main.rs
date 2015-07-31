extern crate rustraytracer;

use std::fs;
use std::io;

use rustraytracer::color::Color;
use rustraytracer::display::{PpmWriter, ImageDisplay};
use rustraytracer::geom::shape::{Mesh};
use rustraytracer::geom::shortcuts::{p, v};
use rustraytracer::scene::{Scene, SceneConfig, CameraConfig, Light,
                           SmoothingFilter, Primitive, Renderer};


#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut scene = Scene::new(
        SceneConfig {
            camera: CameraConfig {
                position: p(0.0, 40.0, 90.0),
                focus_distance: 80.0,
                up: v(0.0, 0.0, -1.0).direction(),
                size: [40.0, 30.0],
                ..Default::default()
            },
            ambient_light: Color::from("#444"),
            background: Color::from("#115"),
        },
    );


    let mut teapot = io::BufReader::new(fs::File::open("./utah.obj").unwrap());
    let mesh = Mesh::from_obj(&mut teapot).unwrap();

    let prim = Primitive::new(mesh, Color::from("#444"));
    scene.add_primitive(prim);
    scene.add_light(Light::new(
        Color::from("#BBB"),
        p(80.0, 80.0, 50.0)));

    let renderer = Renderer::new(scene, [640, 480], SmoothingFilter(1));

    let image = renderer.render();
    let path = "./out.ppm";
    let mut file = io::BufWriter::new(fs::File::create(path).unwrap());
    let mut display = PpmWriter::new(&mut file);
    display.draw(&image).unwrap();
}
