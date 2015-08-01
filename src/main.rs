extern crate rustraytracer;
extern crate rustc_serialize;

use std::fs;
use std::io;
use std::io::{Read};
use std::str::FromStr;

use rustc_serialize::json::Json;
use rustraytracer::color::Color;
use rustraytracer::display::{PpmWriter, ImageDisplay};
use rustraytracer::geom::shape::{Mesh};
use rustraytracer::geom::shortcuts::p;
use rustraytracer::scene::{Scene, Light, SmoothingFilter, Primitive, Renderer};


#[cfg_attr(test, allow(dead_code))]
fn main() {

    let mut scene_json = String::new();
    fs::File::open("./scene.json").unwrap().read_to_string(&mut scene_json).unwrap();
    let mut scene = Scene::from_json(Json::from_str(&scene_json).unwrap()).unwrap();

    let mut teapot = io::BufReader::new(fs::File::open("./utah.obj").unwrap());
    let mesh = Mesh::from_obj(&mut teapot).unwrap();

    let prim = Primitive::new(mesh, Color::from_str("#444").unwrap());
    scene.add_primitive(prim);
    scene.add_light(Light::new(
        Color::from_str("#BBB").unwrap(),
        p(80.0, 80.0, 50.0)));

    let renderer = Renderer::new(scene, [640, 480], SmoothingFilter(1));

    let image = renderer.render();
    let path = "./out.ppm";
    let mut file = io::BufWriter::new(fs::File::create(path).unwrap());
    let mut display = PpmWriter::new(&mut file);
    display.draw(&image).unwrap();
}
