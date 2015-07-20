use geom::shape::Shape;
use color::Color;


pub struct Object {
    shape: Box<Shape>,
    color: Color,
}

impl Object {
    pub fn new<S>(shape: S, color: Color) -> Object
        where S: Shape + 'static {

        Object {
            shape: Box::new(shape),
            color: color
        }
    }

    pub fn shape(&self) -> &Shape {
        &*self.shape
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
