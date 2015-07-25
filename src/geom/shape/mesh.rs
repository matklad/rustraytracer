use std::io;

use geom::ray::Ray;
use super::{Triangle, Shape, Intersection};

pub struct Mesh {
    triangles: Vec<Triangle>
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        Mesh {
            triangles: triangles
        }
    }

    pub fn from_obj(source: &mut io::Read) -> io::Result<Mesh> {
        let mut s: String;
        source.read_to_strign(&s);
    }
}


impl Shape for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.triangles
            .iter()
            .filter_map(|t| t.intersect(ray))
            .min()
    }
}
