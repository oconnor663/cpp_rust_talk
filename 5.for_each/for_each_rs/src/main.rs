use rand::prelude::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

fn vector_of_random_numbers() -> Vec<i32> {
    let len = 100_000;
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    for _ in 0..len {
        v.push(rng.gen_range(0..100));
    }
    v
}

fn serial() {
    let mut v: Vec<i32> = vector_of_random_numbers();
    v.iter_mut().for_each(|x| {
        *x += 1;
    });
}

fn parallel() {
    let mut v: Vec<i32> = vector_of_random_numbers();
    v.par_iter_mut().for_each(|x| {
        *x += 1;
    });
}

fn with_counter() {
    let mut v: Vec<i32> = vector_of_random_numbers();
    let mut sum = 0;
    v.par_iter_mut().for_each(|x| {
        *x += 1;
        sum += *x;
    });
}

fn with_mutex() {
    let mut v: Vec<i32> = vector_of_random_numbers();
    let sum = Mutex::new(0);
    v.par_iter_mut().for_each(|x| {
        *x += 1;
        *sum.lock().unwrap() += *x;
    });
}

fn with_atomic() {
    let mut v: Vec<i32> = vector_of_random_numbers();
    let sum = AtomicI32::new(0);
    v.par_iter_mut().for_each(|x| {
        *x += 1;
        sum.fetch_add(*x, Ordering::Relaxed);
    });
}

fn main() {
    serial();
    parallel();
    with_counter();
    with_mutex();
    with_atomic();
}
