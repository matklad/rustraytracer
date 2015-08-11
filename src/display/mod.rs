use std::io;

mod console;
mod ppm;

use rendering::Image;

pub use self::console::Console;
pub use self::ppm::PpmWriter;

pub trait ImageDisplay<'a> {
    fn draw(&'a mut self, image: &Image) -> io::Result<()>;
}
