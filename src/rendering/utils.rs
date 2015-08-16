use std::ops::{Add, Sub, Div, Mul};

use scene::ScreenPoint;
use super::Pixel;

pub fn to_uniform(resolution: Pixel, point: ScreenPoint) -> ScreenPoint {
    let resolution = ScreenPoint::from(resolution);
    let result = (point * 2.0 - resolution + ScreenPoint::new(1.0, 1.0)) / resolution;
    assert!(result.is_normalized());
    result
}

pub fn from_uniform(resolution: Pixel, point: ScreenPoint) -> ScreenPoint {
    let resolution = ScreenPoint::from(resolution);
    let result = ((point + ScreenPoint::new(1.0, 1.0)) / 2.0) * resolution -
        ScreenPoint::new(0.5, 0.5);
    result
}

impl Sub<ScreenPoint> for ScreenPoint {
    type Output = ScreenPoint;

    fn sub(self, rhs: ScreenPoint) -> ScreenPoint {
        ScreenPoint::new(self.x - rhs.x, self.y - rhs.y)
    }
}


impl Add<ScreenPoint> for ScreenPoint {
    type Output = ScreenPoint;

    fn add(self, rhs: ScreenPoint) -> ScreenPoint {
        ScreenPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}


impl Div<f64> for ScreenPoint {
    type Output = ScreenPoint;

    fn div(self, rhs: f64) -> ScreenPoint {
        ScreenPoint::new(self.x / rhs, self.y / rhs)
    }
}


impl Mul<f64> for ScreenPoint {
    type Output = ScreenPoint;

    fn mul(self, rhs: f64) -> ScreenPoint {
        ScreenPoint::new(self.x * rhs, self.y * rhs)
    }
}


impl Div<ScreenPoint> for ScreenPoint {
    type Output = ScreenPoint;

    fn div(self, rhs: ScreenPoint) -> ScreenPoint {
        ScreenPoint::new(self.x / rhs.x, self.y / rhs.y)
    }
}


impl Mul<ScreenPoint> for ScreenPoint {
    type Output = ScreenPoint;

    fn mul(self, rhs: ScreenPoint) -> ScreenPoint {
        ScreenPoint::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl From<[u32; 2]> for ScreenPoint {
    fn from(xy: [u32; 2]) -> ScreenPoint {
        ScreenPoint::from([xy[0] as f64, xy[1] as f64])
    }
}
