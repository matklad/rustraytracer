use geom::shape;
use color::Color;

pub struct Material {
    pub color: Box<Texture<Color>>,
    pub diffuse: f64,
    pub specular: f64
}


pub trait Texture<T: Copy> {
    fn at(&self, intersection: &shape::Intersection) -> T;
}

pub struct ConstTextute<T: Copy>(T);

impl<T: Copy> Texture<T> for ConstTextute<T> {
    fn at(&self, _: &shape::Intersection) -> T {
        return self.0
    }
}

pub struct CheckBoard3d<T: Copy> {
    black: T,
    white: T,
}

impl<T: Copy> Texture<T> for CheckBoard3d<T> {
    fn at(&self, i: &shape::Intersection) -> T {
        let p = i.point;
        let is_odd = |f| if (f % 2.0 + 2.0) % 2.0 > 1.0 { 1 } else { 0 };
        [self.black, self.white][(is_odd(p[0]) ^ is_odd(p[1]) ^ is_odd(p[2])) as usize]
    }
}


#[derive(RustcDecodable)]
pub struct MaterialConfig {
    color: Color,
    diffuse: f64,
    specular: f64,
}


impl From<MaterialConfig> for Material {
    fn from(config: MaterialConfig) -> Material {
        Material {
            color: Box::new(CheckBoard3d {
                black: config.color,
                white: Color::from("#0A0")
            }),
            diffuse: config.diffuse,
            specular: config.specular
        }
    }
}
