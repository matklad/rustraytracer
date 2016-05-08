use std::ops::{Mul, Add, Div};
use std::str::FromStr;

use rustc_serialize::{Decodable, Decoder};


#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        assert!(r >= 0.0 && g >= 0.0 && b >= 0.0, "{} {} {}", r, g, b);
        Color { r: r, g: g, b: b }
    }

    pub fn grayscale(&self) -> f64 {
        (self.r + self.g + self.b) / 3.0
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, c: f64) -> Color {
        Color::new(self.r * c, self.g * c, self.b * c)
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, c: f64) -> Color {
        Color::new(self.r / c, self.g / c, self.b / c)
    }
}


impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Color, String> {
        if !(s.len() ==  4 || s.len() == 7) {
            return Err("wrong length".to_string());
        }
        if !(s.starts_with("#")) {
            return Err("should start with #".to_string());
        }
        let p = if s.len() == 4 {1} else {2};
        let digits: Result<Vec<_>, _> = (0..3)
            .map(|i| (1 + i*p, 1 + (i + 1)*p))
            .map(|(l, r)| &s[l..r])
            .map(|s| u8::from_str_radix(s, 16))
            .collect();

        match digits {
            Err(_) => Err("bad digits".to_string()),
            Ok(v) => {
                let parts = v.iter().map(|&i| if p == 1 {i * 17} else {i})
                    .map(|i| i as f64 / 255.0)
                    .collect::<Vec<_>>();
                Ok(Color::new(parts[0], parts[1], parts[2]))
            }
        }
    }
}

impl From<&'static str> for Color {
    fn from(s: &'static str) -> Color {
        Color::from_str(s).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_hex() {
        let c = Color::from("#FFF");
        assert!(c.r > 0.9);
        assert!(c.g > 0.9);
        assert!(c.b > 0.9);
    }
}


pub mod palette {
    use super::Color;
    const EPS: f64 = 0.05;

    pub const RED: Color = Color {r: 1.0, g: EPS, b: EPS};
    pub const GREEN: Color = Color {r: EPS, g: 1.0, b: EPS};
    pub const BLUE: Color = Color {r: EPS, g: EPS, b: 1.0};
}


#[derive(Debug, Clone, Copy)]
pub struct Rgb8Bit {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Rgb8Bit {
    pub fn truncate(color: &Color) -> Rgb8Bit {
        fn bound(f: f64, low: f64, hi: f64) -> f64 {
            assert!(low <= hi);
            f.max(low).min(hi)
        }

        let to_u8 = |f| bound(f * 255.0, 0.0, 255.0).round() as u8;

        Rgb8Bit {
            r: to_u8(color.r),
            g: to_u8(color.g),
            b: to_u8(color.b),
        }
    }

}

impl Decodable for Color {
    fn decode<D: Decoder>(d: &mut D) -> Result<Color, D::Error> {
        let s: String = try!(Decodable::decode(d));
        Color::from_str(&s).map_err(|_| d.error("bad color"))
    }
}
