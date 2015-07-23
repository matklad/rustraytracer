use std::ops::{Mul, Add, Div};


#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r: r, g: g, b: b}
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

impl From<&'static str> for Color {
    fn from(s: &str) -> Color {
        assert!(s.len() ==  4 || s.len() == 7);
        assert!(s.starts_with("#"));
        let p = if s.len() == 4 {1} else {2};
        let parts = (0..3)
            .map(|i| (1 + i*p, 1 + (i + 1)*p))
            .map(|(l, r)| &s[l..r])
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .map(|i| if p == 1 {i * 17} else {i})
            .map(|i| i as f64 / 255.0)
            .collect::<Vec<_>>();
        Color::new(parts[0], parts[1], parts[2])
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
