extern crate rustraytracer;
extern crate rustc_serialize;

use std::fs;
use std::io;
use std::io::{Read};

use rustc_serialize::json::Json;
use rustraytracer::display::{PpmWriter, ImageDisplay};
use rustraytracer::scene::{Scene, SmoothingFilter, Renderer};


#[cfg_attr(test, allow(dead_code))]
fn main() {

    let mut scene_json = String::new();
    fs::File::open("./scene.json").unwrap().read_to_string(&mut scene_json).unwrap();
    let scene = Scene::from_json(Json::from_str(&scene_json).unwrap()).unwrap();


    let renderer = Renderer::new(scene, [640, 480], SmoothingFilter(1));

    let image = renderer.render();
    let path = "./out.ppm";
    let mut file = io::BufWriter::new(fs::File::create(path).unwrap());
    let mut display = PpmWriter::new(&mut file);
    display.draw(&image).unwrap();
}
