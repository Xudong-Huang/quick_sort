use quick_sort::quick_sort;
use rand::{distributions::Standard, seq::SliceRandom, Rng};
use std::time;

fn default_vec(n: usize) -> Vec<u32> {
    rand::thread_rng().sample_iter(Standard).take(n).collect()
}

fn main() {
    let mut v = Vec::from_iter(0..20);
    v.shuffle(&mut rand::thread_rng());
    println!("v = {:?}", v);
    quick_sort(&mut v);
    println!("v = {:?}", v);

    let n = 10_000_000;

    let mut v = default_vec(n);
    let mut v1 = v.clone();

    let start = time::Instant::now();
    v.sort();
    let dur = start.elapsed();
    println!("seq sorted {} ints: {} s", n, dur.as_secs_f32());

    let start = time::Instant::now();
    quick_sort(&mut v1);
    let dur = start.elapsed();
    println!("par sorted {} ints: {} s", n, dur.as_secs_f32());

    assert_eq!(v, v1);
}
