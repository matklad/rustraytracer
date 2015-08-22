use geom::shape;
use color::Color;
use super::config::{MaterialConfig, TextureConfig};

pub struct Material {
    pub color: Box<Texture<Color>>,
    pub diffuse: f64,
    pub specular: f64,
    pub reflectance: f64,
}


pub trait Texture<T: Copy + Send + Sync>: Send + Sync {
    fn at(&self, intersection: &shape::Intersection) -> T;
}

pub struct ConstTextute<T: Copy>(T);

impl<T: Copy + Send + Sync> Texture<T> for ConstTextute<T> {
    fn at(&self, _: &shape::Intersection) -> T {
        return self.0
    }
}

pub struct Checkboard3d<T: Copy> {
    black: T,
    white: T,
}

impl<T: Copy + Send + Sync> Texture<T> for Checkboard3d<T> {
    fn at(&self, i: &shape::Intersection) -> T {
        let p = i.point;
        let is_odd = |f| if (f % 2.0 + 2.0) % 2.0 > 1.0 { 1 } else { 0 };
        [self.black, self.white][(is_odd(p[0]) ^ is_odd(p[1]) ^ is_odd(p[2])) as usize]
    }
}


impl From<MaterialConfig> for Material {
    fn from(config: MaterialConfig) -> Material {
        let color: Box<Texture<Color>> = match config.texture {
            TextureConfig::Checkboard3d(black, white) => Box::new(
                Checkboard3d {
                    black: black,
                    white: white
                }),
            TextureConfig::Color(c) => Box::new(ConstTextute(c))
        };

        Material {
            color: color,
            diffuse: config.diffuse,
            specular: config.specular,
            reflectance: config.reflectance,
        }
    }
}
