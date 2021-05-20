use rand::prelude::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Mutex, MutexGuard};

fn vector_of_ints() -> Vec<i32> {
    let len = 100_000;
    let mut rng = rand::thread_rng();
    let mut v = Vec::new();
    for _ in 0..len {
        v.push(rng.gen_range(0..100));
    }
    v
}

fn loop_() {
    let mut v: Vec<i32> = vector_of_ints();
    for x in &mut v {
        *x += 1;
    }
}

fn serial() {
    let mut v: Vec<i32> = vector_of_ints();
    v.iter_mut().for_each(|x| {
        *x += 1;
    });
}

fn parallel() {
    let mut v: Vec<i32> = vector_of_ints();
    v.par_iter_mut().for_each(|x| {
        *x += 1;
    });
}

fn loop_counter() {
    let mut v: Vec<i32> = vector_of_ints();
    let mut sum = 0;
    for x in &mut v {
        *x += 1;
        sum += *x;
    }
    dbg!(sum);
}

fn serial_counter() {
    let mut v: Vec<i32> = vector_of_ints();
    let mut sum = 0;
    v.iter_mut().for_each(|x| {
        *x += 1;
        sum += *x;
    });
    dbg!(sum);
}

fn parallel_counter() {
    let mut v: Vec<i32> = vector_of_ints();
    let mut sum = 0;
    v.par_iter_mut().for_each(|x| {
        *x += 1;
        sum += *x;
    });
    dbg!(sum);
}

fn parallel_atomic() {
    let mut v: Vec<i32> = vector_of_ints();
    let sum: AtomicI32 = AtomicI32::new(0);
    v.par_iter_mut().for_each(|x| {
        *x += 1;
        sum.fetch_add(*x, Ordering::Relaxed);
    });
    dbg!(sum);
}

fn parallel_mutex() {
    let mut v: Vec<i32> = vector_of_ints();
    let sum: Mutex<i32> = Mutex::new(0);
    v.par_iter_mut().for_each(|x| {
        *x += 1;
        let mut guard: MutexGuard<i32> =
            sum.lock().unwrap();
        *guard += *x;
    });
    dbg!(sum);
}

fn main() {
    loop_();
    serial();
    parallel();
    loop_counter();
    serial_counter();
    parallel_counter();
    parallel_atomic();
    parallel_mutex();
}
