mod bvh;
mod obj_parser;


use std::error::Error;
use std::io;

use ray::Ray;
use super::{Triangle, Shape, Intersection};
use self::bvh::Bvh;
use self::obj_parser::ObjParser;

pub struct Mesh {
    index: Bvh<Triangle>
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        Mesh {
            index: Bvh::new(triangles)
        }
    }

    pub fn from_obj(source: &mut io::Read) -> Result<Mesh, Box<Error>> {
        ObjParser::new().parse(source)
    }
}


impl Shape for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.index.intersect(ray)
    }
}
