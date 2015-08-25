extern crate time;
extern crate rand;

pub mod datastructures;
pub mod props;

pub fn time_it<F, T>(f: F) -> (T, f64) where F: FnOnce() -> T {
    let start = time::precise_time_s();
    let result = f();
    let end = time::precise_time_s();
    return (result, end - start)
}
