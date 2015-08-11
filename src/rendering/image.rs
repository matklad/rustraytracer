use std::ops::{Index, IndexMut};
use std::iter;

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
            (0..self.width()).map(move |x| ((x, y), self[[x, y]]))
        }))
    }
}

impl Index<Pixel> for Image {
    type Output = Color;

    fn index(&self, index: Pixel) -> &Color {
        let (x, y) = (index[0], index[1]);
        assert!(x < self.width() && y < self.height());
        &self.pixels[x as usize][y as usize]
    }
}

impl IndexMut<Pixel> for Image {
    fn index_mut(&mut self, index: Pixel) -> &mut Color {
        let (x, y) = (index[0], index[1]);
        assert!(x < self.width() && y < self.height());
        &mut self.pixels[x as usize][y as usize]
    }
}


pub fn new_image(resolution: Pixel) -> Image {

    let pixels = (0..resolution[0]).map(|_| {
        iter::repeat(Color::new(0.0, 0.0, 0.0)).take(resolution[1] as usize)
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    Image {
        resolution: resolution,
        pixels: pixels
    }
}
