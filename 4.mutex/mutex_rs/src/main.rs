use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use std::thread;

fn stack_local() {
    let my_string: Mutex<String> = Mutex::new(String::new());
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let thread_handle = thread::spawn(|| {
            let mut guard: MutexGuard<String> = my_string.lock().unwrap();
            guard.push_str("some characters");
        });
        thread_handles.push(thread_handle);
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    dbg!(my_string);
}

fn with_shared_ptr() {
    let my_string: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let arc_clone = my_string.clone();
        let thread_handle = thread::spawn(move || {
            let mut guard = arc_clone.lock().unwrap();
            guard.push_str("some characters");
        });
        thread_handles.push(thread_handle);
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    dbg!(my_string);
}

fn forgot_mutex() {
    let my_string: Arc<String> = Arc::new(String::new());
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let mut arc_clone = my_string.clone();
        let thread_handle = thread::spawn(move || {
            arc_clone.push_str("some characters");
        });
        thread_handles.push(thread_handle);
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    dbg!(my_string);
}

fn with_shared_mutex() {
    let my_string: Arc<RwLock<String>> = Arc::new(RwLock::new(String::new()));
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let arc_clone = my_string.clone();
        let thread_handle = thread::spawn(move || {
            let mut guard = arc_clone.read().unwrap();
            guard.push_str("some characters");
        });
        thread_handles.push(thread_handle);
    }
    for thread_handle in thread_handles {
        thread_handle.join().unwrap();
    }
    dbg!(my_string);
}

fn smuggle() {
    let my_string: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let arc_clone = my_string.clone();
        let thread_handle = thread::spawn(move || {
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

fn main() {
    stack_local();
    with_shared_ptr();
    forgot_mutex();
    with_shared_mutex();
    smuggle();
}
