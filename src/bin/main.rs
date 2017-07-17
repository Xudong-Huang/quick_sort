extern crate may;
extern crate rand;
extern crate quick_sort;

use std::time;
use quick_sort::*;
use rand::{Rng, SeedableRng, XorShiftRng};

fn default_vec(n: usize) -> Vec<u32> {
    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
    (0..n).map(|_| rng.next_u32()).collect()
}

fn main() {
    may::config().set_workers(4);
    let mut v = vec![8u32, 2, 9, 6, 5, 0, 1, 4, 3, 7];
    quick_sort(&mut v);
    println!("v = {:?}", v);

    let n = 10000000;

    let mut v = default_vec(n);
    let start = time::Instant::now();
    v.sort();
    let dur = start.elapsed();
    let nanos = dur.subsec_nanos() as u64 + dur.as_secs() * 1_000_000_000u64;
    println!("seq sorted {} ints: {} s", n, nanos as f32 / 1e9f32);

    let mut v1 = default_vec(n);
    let start = time::Instant::now();
    quick_sort(&mut v1);
    let dur = start.elapsed();
    let nanos = dur.subsec_nanos() as u64 + dur.as_secs() * 1_000_000_000u64;
    println!("par sorted {} ints: {} s", n, nanos as f32 / 1e9f32);

    assert_eq!(v, v1);
}
