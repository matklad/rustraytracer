use color::Color;
use geom::{Point};


pub struct LightSource {
    source: Box<LightSourceImpl>
}

impl LightSource {
    pub fn position(&self) -> Point {
        self.source.position()
    }

    pub fn illuminate(&self, p: Point) -> Color {
        let distance = (p - self.position()).length();
        return self.source.outgoing_light(p) / distance.sqrt()
    }
}


#[derive(Debug, RustcDecodable)]
pub struct LightConfig {
    color: Color,
    intensity: f64,
    position: Point
}


impl From<LightConfig> for LightSource {
    fn from(config: LightConfig) -> LightSource {
        LightSource {
            source: Box::new(PointLight {
                color: config.color,
                intensity: config.intensity,
                position: config.position })
        }
    }
}


trait LightSourceImpl {
    fn position(&self) -> Point;
    fn outgoing_light(&self, p: Point) -> Color;
}


#[derive(Debug, RustcDecodable)]
struct PointLight {
    color: Color,
    intensity: f64,
    position: Point
}


impl LightSourceImpl for PointLight {
    fn position(&self) -> Point {
        self.position
    }

    fn outgoing_light(&self, _p: Point) -> Color {
        self.color * self.intensity
    }
}
