use color::Color;
use geom::{Point, UnitVector, Dot};
use super::config::{LightConfig, LightKind};


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
        let v = p - self.position();
        let distance = v.length();
        let direction = v.direction();
        let coef = self.intensity * self.source.intensity_at(direction) / distance.sqrt();
        return self.color * coef
    }
}


impl From<LightConfig> for LightSource {
    fn from(config: LightConfig) -> LightSource {
        let source: Box<LightSourceImpl> = match config.kind {
            LightKind::PointLight => Box::new(PointLight),
            LightKind::SpotLight {look_at, inner_angle, outer_angle} => {
                assert!(inner_angle <= outer_angle);
                let direction = (look_at - config.position).direction();
                Box::new(SpotLight {
                    direction: direction,
                    inner_cos: inner_angle.cos(),
                    outer_cos: outer_angle.cos(),
                })
            }
        };
        LightSource {
            color: config.color,
            intensity: config.intensity,
            position: config.position,
            source: source,
        }
    }
}


trait LightSourceImpl: Send + Sync {
    fn intensity_at(&self, d: UnitVector) -> f64;
}


struct PointLight;


impl LightSourceImpl for PointLight {
    fn intensity_at(&self, _d: UnitVector) -> f64 {
        1.0
    }
}

struct SpotLight {
    direction: UnitVector,
    outer_cos: f64,
    inner_cos: f64,
}

impl SpotLight {
    fn cos(&self, d: UnitVector) -> f64 {
        return self.direction.dot(d)
    }
}

impl LightSourceImpl for SpotLight {
    fn intensity_at(&self, d: UnitVector) -> f64 {
        let cos = self.cos(d);
        if cos < self.outer_cos {
            return 0.0;
        }

        if self.inner_cos < cos {
            return 1.0;
        }
        let t = (self.outer_cos - cos) / (self.outer_cos - self.inner_cos);
        assert!(0.0 <= t && t <= 1.0);
        t
    }
}
