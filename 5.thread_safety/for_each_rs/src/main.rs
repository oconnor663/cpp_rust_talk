use rayon::prelude::*;
use std::sync::Mutex;

fn serial() {
    let mut my_string = String::new();
    let my_vector = vec!["some", "words", "here"];
    my_vector.iter().for_each(|s| {
        my_string.push_str(s);
    });
}

fn parallel() {
    let mut my_string = String::new();
    let my_vector = vec!["some", "words", "here"];
    my_vector.par_iter().for_each(|s| {
        my_string.push_str(s);
    });
}

fn with_mutex() {
    let my_string = Mutex::new(String::new());
    let my_vector = vec!["some", "words", "here"];
    my_vector.par_iter().for_each(|s| {
        my_string.lock().unwrap().push_str(s);
    });
}

fn main() {
    serial();
    parallel();
    with_mutex();
}
