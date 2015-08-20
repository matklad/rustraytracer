use color::Color;
use geom::{Point};


pub struct LightSource {
    color: Color,
    intensity: f64,
    position: Point,
    source: Box<LightSourceImpl>,
}

impl LightSource {
    pub fn position(&self) -> Point {
        self.position
    }

    pub fn illuminate(&self, p: Point) -> Color {
        let distance = (p - self.position()).length();
        let coef = self.intensity * self.source.intensity_at(p) / distance.sqrt();
        return self.color * coef
    }
}


#[derive(Debug, RustcDecodable)]
pub struct LightConfig {
    color: Color,
    intensity: f64,
    position: Point,
}


impl From<LightConfig> for LightSource {
    fn from(config: LightConfig) -> LightSource {
        LightSource {
            color: config.color,
            intensity: config.intensity,
            position: config.position,
            source: Box::new(PointLight),
        }
    }
}


trait LightSourceImpl {
    fn intensity_at(&self, p: Point) -> f64;
}


struct PointLight;


impl LightSourceImpl for PointLight {
    fn intensity_at(&self, _p: Point) -> f64 {
        1.0
    }
}
