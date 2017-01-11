use std::io;

use rendering::Image;
use color::Rgb8Bit;
use super::ImageDisplay;


pub struct PpmWriter<'a> {
    destination: &'a mut (io::Write + 'a)
}

impl<'a> PpmWriter<'a> {
    pub fn new(destination: &'a mut io::Write) -> PpmWriter<'a> {
        PpmWriter {
            destination: destination
        }
    }
}


impl<'a> ImageDisplay<'a> for PpmWriter<'a> {
    fn draw(&'a mut self, image: &Image) -> io::Result<()> {
        let magic_number = "P3";
        let max_color = 255;
        write!(&mut self.destination, "{}\n{} {}\n{}\n",
               magic_number, image.width(), image.height(), max_color)?;

        for (xy, color) in image.iter() {
            if xy[0] == 0 {
                write!(&mut self.destination, "\n")?;
            }
            let Rgb8Bit { r, g, b } = Rgb8Bit::truncate(&color);
            write!(&mut self.destination, "{:3} {:3} {:3}  ",
                   r, g, b)?;
        }
        Ok(())
    }
}
