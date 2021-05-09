#![allow(unused_variables)]
#![allow(dead_code)]

use std::sync::{Arc, Mutex, RwLock};

fn good() {
    let my_string = Arc::new(Mutex::new(String::new()));
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let arc_clone = my_string.clone();
        let thread_handle = std::thread::spawn(move || {
            let mut guard = arc_clone.lock().unwrap();
            let my_string_reference: &mut String = &mut *guard;
            my_string_reference.push_str("some characters");
        });
        thread_handles.push(thread_handle);
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    dbg!(my_string);
}

fn bad() {
    let my_string = Arc::new(Mutex::new(String::new()));
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let arc_clone = my_string.clone();
        let thread_handle = std::thread::spawn(move || {
            let mut guard = arc_clone.lock().unwrap();
            let my_string_reference: &mut String = &mut *guard;
            drop(guard);
            my_string_reference.push_str("some characters");
        });
        thread_handles.push(thread_handle);
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    dbg!(my_string);
}

fn rwlock() {
    let my_string = Arc::new(RwLock::new(String::new()));
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let arc_clone = my_string.clone();
        let thread_handle = std::thread::spawn(move || {
            let guard = arc_clone.read().unwrap();
            let my_string_reference: &mut String = &mut *guard;
            my_string_reference.push_str("some characters");
        });
        thread_handles.push(thread_handle);
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    dbg!(my_string);
}

fn main() {
    good();
    bad();
    rwlock();
}
