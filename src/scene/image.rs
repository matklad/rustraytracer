use color::Color;


pub type Pixel = [u32; 2];

#[derive(Debug)]
pub struct Image {
    resolution: Pixel,
    pixels: Vec<Vec<Color>>
}

impl Image {
    pub fn width(&self) -> u32 {
        self.resolution[0]
    }

    pub fn height(&self) -> u32 {
        self.resolution[1]
    }

    pub fn iter<'a>(&'a self) -> Box<Iterator<Item=((u32, u32), Color)> + 'a> {
        Box::new((0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| ((x, y), self.at(x, y)))
        }))
    }

    pub fn at(&self, x: u32, y: u32) -> Color {
        assert!(x < self.width() && y < self.height());
        self.pixels[x as usize][y as usize]
    }
}

pub fn new_image<F>(resolution: Pixel, f: F) -> Image where F: Fn(u32, u32) -> Color {

    let pixels = (0..resolution[0]).map(|x| {
        (0..resolution[1]).map(|y| f(x, y))
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    Image {
        resolution: resolution,
        pixels: pixels
    }
}
