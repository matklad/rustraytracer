use color::Color;
use geom::{Point};

#[derive(Debug, RustcDecodable)]
pub struct Light {
    color: Color,
    position: Point
}

impl Light {
    pub fn position(&self) -> Point {
        self.position
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
