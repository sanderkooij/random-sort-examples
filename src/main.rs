use rand::prelude::*;
use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let vec = make_random_vec(1_000_000, 100);
    for _ in 0..2500 {
        let mut v = vec.clone();
        v.sort_unstable();
        //v.sort();
    }
    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

pub fn make_random_vec(sz: usize, modulus: i64) -> Vec<i64> {
    let mut v: Vec<i64> = Vec::with_capacity(sz);
    for _ in 0..sz {
        let x: i64 = random();
        v.push(x % modulus);
    }
    v
}
