use std::io;
use scene;

mod console;
mod ppm;

pub use self::console::Console;
pub use self::ppm::PpmWriter;

pub trait ImageDisplay<'a> {
    fn draw(&'a mut self, image: &scene::Image) -> io::Result<()>;
}
