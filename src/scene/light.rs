use color::Color;
use geom::{Point};

pub struct Light {
    color: Color,
    position: Point
}

impl Light {
    pub fn new(color: Color, position: Point) -> Light {
        Light {
            color: color,
            position: position
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
