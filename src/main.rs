extern crate rustraytracer;
extern crate rustc_serialize;
extern crate regex;
extern crate time;

use std::fs;
use regex::Regex;
use std::io::{self, Read};

use rustc_serialize::json;
use rustraytracer::display::{PpmWriter, ImageDisplay};
use rustraytracer::scene::{Scene, SceneConfig};
use rustraytracer::rendering::{Tracer, TracerConfig};
use rustraytracer::utils::time_it;

#[derive(Debug, RustcDecodable)]
struct Config {
    scene: SceneConfig,
    rendering: TracerConfig,
}

fn read_scene_description(path: &str) -> String {
    let mut result = String::new();
    fs::File::open(path).unwrap().read_to_string(&mut result).unwrap();
    let comment = Regex::new(r"(?m)^\s*//.*$").unwrap();
    comment.replace_all(&result, "\n")
}


#[cfg_attr(test, allow(dead_code))]
fn main() {
    println!("Start rendering...");
    let start = time::precise_time_s();
    let ((scene, conf), prep_time) = time_it(|| {
        let conf: Config = json::decode(&read_scene_description("./scenes/buddha.json")).unwrap();
        let scene = Scene::new(conf.scene).unwrap();
        (scene, conf.rendering)
    });
    let tracer = Tracer::new(scene, conf);

    let (image, stats) = tracer.render();
    let path = "./out.ppm";
    let mut file = io::BufWriter::new(fs::File::create(path).unwrap());
    let mut display = PpmWriter::new(&mut file);
    display.draw(&image).unwrap();

    let end = time::precise_time_s();
    println!("\nPreprocess:  {:.2}s\n{}\n\nTotal: {:.2} seconds",
             prep_time, stats, end - start);
}
