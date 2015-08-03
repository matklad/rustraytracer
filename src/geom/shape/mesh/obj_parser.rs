use std::{io, fmt};
use std::error::Error;

use geom::{Point, UnitVector, Vector};
use geom::shape::Triangle;
use geom::shape::mesh::Mesh;


#[derive(Debug)]
pub struct ParseObjError;

impl Error for ParseObjError {
    fn description(&self) -> &str {
        "Invalid .obj file"
    }
}

impl fmt::Display for ParseObjError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}


pub struct ObjParser {
    pub points: Vec<Point>,
    pub normals: Vec<UnitVector>,
    pub faces: Vec<Triangle>,
}

impl ObjParser {
    pub fn new() -> ObjParser {
        ObjParser {
            points: Vec::new(),
            normals: Vec::new(),
            faces: Vec::new()
        }
    }

    pub fn parse(mut self, source: &mut io::Read) -> Result<Mesh, Box<Error>> {
        let mut s = String::new();
        try!(source.read_to_string(&mut s));
        for line in s.lines() {
            if line.starts_with("v ") {
                try!(self.parse_vertex(line));
            } else if line.starts_with("vn ") {
                try!(self.parse_normal(line));
            } else if line.starts_with("f ") {
                try!(self.parse_face(line));
            }
        }

        Ok(Mesh::new(self.faces))
    }

    fn parse_vertex(&mut self, s: &str) -> Result<(), Box<Error>> {
        let coords = try!(ObjParser::parse_coordinates(s));
        self.points.push(Point::new(coords.0, coords.1, coords.2));
        Ok(())
    }

    fn parse_normal(&mut self, s: &str) -> Result<(), Box<Error>> {
        let coords = try!(ObjParser::parse_coordinates(s));
        self.normals.push(Vector::new(coords.0, coords.1, coords.2).direction());
        Ok(())
    }

    fn parse_face(&mut self, s: &str) -> Result<(), Box<Error>> {
        fn read_group(s: &str) -> Result<(usize, usize, usize), Box<Error>> {
            let inds = try!(s.split('/')
                            .map(|i| i.parse::<usize>().map(|x| x - 1))
                            .collect::<Result<Vec<_>, _>>());
            if inds.len() != 3 {
                return Err(Box::new(ParseObjError));
            }
            Ok((inds[0], inds[1], inds[2]))
        }

        let verts = try!(s.split_whitespace()
                         .skip(1)
                         .map(read_group)
                         .collect::<Result<Vec<_>, _>>());

        if verts.len() != 3 {
            return Err(Box::new(ParseObjError));
        }
        let f = Triangle::with_normals(
            self.points[verts[0].0], self.points[verts[1].0], self.points[verts[2].0],
            [self.normals[verts[0].2], self.normals[verts[1].2], self.normals[verts[2].2]]);

        self.faces.push(f);
        Ok(())
    }

    fn parse_coordinates(s: &str) -> Result<(f64, f64, f64), Box<Error>> {
        let coords = try!(s.split_whitespace()
                          .skip(1)
                          .map(|s| s.parse::<f64>())
                          .collect::<Result<Vec<_>, _>>());
        if coords.len() != 3 {
            return Err(Box::new(ParseObjError));
        }

        Ok((coords[0], coords[1], coords[2]))
    }
}
