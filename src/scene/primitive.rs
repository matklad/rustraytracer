use geom::shape::Shape;

use super::material::Material;

pub struct Primitive {
    pub shape: Box<Shape>,
    pub material: Material
}

impl Primitive {
    pub fn new<S>(shape: S, material: Material) -> Primitive
        where S: Shape + 'static {

        Primitive {
            shape: Box::new(shape),
            material: material
        }
    }
}
