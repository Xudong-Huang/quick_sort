extern crate may;
extern crate num_cpus;
extern crate quick_sort;
extern crate rand;

use quick_sort::*;
use rand::{Rng, SeedableRng, XorShiftRng};
use std::time;

fn default_vec(n: usize) -> Vec<u32> {
    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
    (0..n).map(|_| rng.next_u32()).collect()
}

fn is_sorted<T: Send + Ord>(v: &[T]) -> bool {
    (1..v.len()).all(|i| v[i - 1] <= v[i])
}

fn main() {
    may::config().set_workers(num_cpus::get());
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

    assert_eq!(is_sorted(&v1), true);
}
