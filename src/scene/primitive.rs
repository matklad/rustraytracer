use geom::shape::Shape;
use color::Color;


pub struct Primitive {
    shape: Box<Shape>,
    color: Color,
}

impl Primitive {
    pub fn new<S>(shape: S, color: Color) -> Primitive
        where S: Shape + 'static {

        Primitive {
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
