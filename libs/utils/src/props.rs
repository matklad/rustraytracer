use rand;

const N_CHECKS: i32 = 1000;

pub fn check_prop2<A, B, F>(mut prop: F)
    where A: rand::Rand, B: rand::Rand,
          F: FnMut(A, B) -> () {

    for _ in 0..N_CHECKS {
        let a = rand::random::<A>();
        let b = rand::random::<B>();
        prop(a, b);
    }
}


pub fn check_prop<A, F>(mut prop: F)
    where A: rand::Rand,
          F: FnMut(A) -> () {

    for _ in 0..N_CHECKS {
        let a = rand::random::<A>();
        prop(a);
    }
}
