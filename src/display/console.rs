use std::io;

use rendering::Image;
use super::ImageDisplay;


pub struct Console {
    black: char,
    white: char,
}

impl Console {
    pub fn new(black: char, white: char) -> Console {
        Console {
            black: black,
            white: white,
        }
    }
}


impl<'a> ImageDisplay<'a> for Console {
    fn draw(&mut self, image: &Image) -> io::Result<()> {

        for ((x, _), color) in image.iter() {
            if x == 0 {
                println!("");
            }
            let color = if color.grayscale() < 0.5 {
                self.black
            } else {
                self.white
            };
            print!("{}", color);
        }

        Ok(())
    }
}
