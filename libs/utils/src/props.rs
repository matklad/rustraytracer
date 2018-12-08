use rand;
use rand::distributions::{Distribution, Standard};

const N_CHECKS: i32 = 1000;

pub fn check_prop2<A, B, F>(mut prop: F)
    where Standard: Distribution<A>,
          Standard: Distribution<B>,
          F: FnMut(A, B) -> () {

    for _ in 0..N_CHECKS {
        let a = rand::random::<A>();
        let b = rand::random::<B>();
        prop(a, b);
    }
}


pub fn check_prop<A, F>(mut prop: F)
    where Standard: Distribution<A>,
          F: FnMut(A) -> () {

    for _ in 0..N_CHECKS {
        let a = rand::random::<A>();
        prop(a);
    }
}
