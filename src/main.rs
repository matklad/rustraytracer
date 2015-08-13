extern crate rustraytracer;
extern crate rustc_serialize;

use std::fs;
use std::io::{self, Read};

use rustc_serialize::json;
use rustraytracer::display::{PpmWriter, ImageDisplay};
use rustraytracer::scene::{Scene, SceneConfig};
use rustraytracer::rendering::{Renderer, RendererConfig};

#[derive(RustcDecodable)]
struct Config {
    scene: SceneConfig,
    rendering: RendererConfig,
}


#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut scene_json = String::new();
    fs::File::open("./scene.json").unwrap().read_to_string(&mut scene_json).unwrap();
    let conf: Config = json::decode(&scene_json).unwrap();
    let scene = Scene::new(conf.scene).unwrap();
    let renderer = Renderer::new(&scene, conf.rendering);

    let image = renderer.render();
    let path = "./out.ppm";
    let mut file = io::BufWriter::new(fs::File::create(path).unwrap());
    let mut display = PpmWriter::new(&mut file);
    display.draw(&image).unwrap();
}
