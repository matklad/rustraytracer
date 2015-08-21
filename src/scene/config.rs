use std::collections::HashMap;
use std::error::Error;
use std::{fmt, fs, io};
use std::rc::Rc;

use color::Color;
use geom::{Point, UnitVector};
use geom::shape::{Mesh, Plane, Sphere};
use super::primitive::Primitive;
use super::material::Material;


#[derive(Debug, RustcDecodable)]
pub struct SceneConfig {
    pub camera: CameraConfig,
    pub ambient_light: Color,
    pub background_color: Color,
    pub materials: HashMap<String, MaterialConfig>,
    pub primitives: Vec<PrimitiveConfig>,
    pub lights: Vec<LightConfig>
}


#[derive(Debug, RustcDecodable)]
pub struct CameraConfig {
    pub position: Point,
    pub look_at: Point,
    pub focus_distance: f64,
    pub up: UnitVector,

    pub size: [f64; 2],
}


#[derive(Debug, RustcDecodable)]
pub struct MaterialConfig {
    pub specular: f64,
    pub diffuse: f64,
    pub texture: TextureConfig,
    pub reflectance: f64,
}


#[derive(Debug, RustcDecodable)]
pub enum TextureConfig {
    Checkboard3d(Color, Color),
    Color(Color)
}


#[derive(Debug, RustcDecodable)]
pub struct PrimitiveConfig {
    material: String,
    kind: PrimitiveKind,
}


#[derive(Debug, RustcDecodable)]
pub enum PrimitiveKind {
    Mesh {
        location: String
    },
    Plane {
        position: Point,
        normal: UnitVector
    },
    Sphere {
        position: Point,
        radius: f64
    },
}

#[derive(Debug)]
pub struct ParseSceneError {
    description: String
}

impl Error for ParseSceneError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for ParseSceneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

pub fn read_primitive<'a>(conf: PrimitiveConfig, materials: &HashMap<String, Rc<Material>>)
                  -> Result<Primitive, Box<Error>> {
    let material = try!(materials.get(&conf.material).ok_or(ParseSceneError {
        description: format!("No such material: {}", conf.material)
    }));
    let material = material.clone();

    match conf.kind {
        PrimitiveKind::Mesh {location} => {
            let mut file = try!(fs::File::open(&location).map(io::BufReader::new));
            let mesh = try!(Mesh::from_obj(&mut file));
            Ok(Primitive::new(mesh, material))
        },
        PrimitiveKind::Plane {position, normal} =>
            Ok(Primitive::new(Plane::new(position, normal), material)),
        PrimitiveKind::Sphere {position, radius} =>
            Ok(Primitive::new(Sphere::new(position, radius), material))
    }
}

#[derive(Debug, RustcDecodable)]
pub struct LightConfig {
    pub color: Color,
    pub intensity: f64,
    pub position: Point,
    pub kind: LightKind,
}

#[derive(Debug, RustcDecodable)]
pub enum LightKind {
    PointLight,
    SpotLight {
        look_at: Point,
        inner_angle: f64,
        outer_angle: f64,
    },
}
