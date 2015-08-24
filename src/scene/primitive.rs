use std::cmp::Ordering;

use geom::shape::{self, Shape};
use super::material::Material;

pub struct Primitive {
    pub shape: Box<Shape>,
    pub material_idx: usize,
}

impl Primitive {
    pub fn new<S>(shape: S, material_idx: usize) -> Primitive
        where S: Shape + 'static
    {
        Primitive {
            shape: Box::new(shape),
            material_idx: material_idx
        }
    }
}


#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub geom: shape::Intersection,
    pub material: &'a Material
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Intersection<'a>) -> Ordering {
        self.geom.cmp(&other.geom)
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Intersection<'a>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Intersection<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for Intersection<'a> {}
