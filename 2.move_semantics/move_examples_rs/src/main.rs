#![allow(unused_variables)]

use std::fs::File;
use std::sync::Mutex;

mod regular {
    fn f(s1: &mut String) {
        let s2 = *s1;
        dbg!(s2);
    }

    fn g() {
        let mut s1 = "foo".to_string();
        f(&mut s1);
        dbg!(s1);
    }
}

mod swap {
    use std::mem;

    fn f(s1: &mut String) {
        let mut s2 = "".to_string();
        mem::swap(s1, &mut s2);
        dbg!(s2);
    }

    fn g() {
        let mut s1 = "foo".to_string();
        f(&mut s1);
        dbg!(s1);
    }
}

mod option {
    fn f(s1: &mut Option<String>) {
        let s2 = s1.take().unwrap();
        dbg!(s2);
    }

    fn g() {
        let mut s1: Option<String> =
            Some("foo".to_string());
        f(&mut s1);
        dbg!(s1);
    }
}

mod vector {
    fn f(v: &mut Vec<String>) {
        let s2 = v.remove(0);
        dbg!(s2);
    }

    fn g() {
        let mut v = vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ];
        f(&mut v);
        dbg!(v);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
        let s2 = s1;
        let mut v = Vec::new();
        v.push(s2);
    }
    {
        let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
        let s2 = s1.clone();
        let mut v = Vec::new();
        v.push(s2.clone());
    }
    {
        let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
        let s2 = s1;
        dbg!(s1);
    }
    {
        let s1 = "abcde".to_string();
        let my_view = s1.as_str();
        let s2 = s1;
        dbg!(my_view);
    }
    {
        let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
        let s2 = s1;
        let mut v = Vec::new();
        v.push(s2);
        let s3 = v[0];
    }
    {
        let file = File::open("/dev/null")?;
        drop(file);
    }
    {
        let mutex1 = Mutex::new(0);
        let mutex2 = mutex1;
    }
    {
        let mutex = Mutex::new(0);
        let guard1 = mutex.lock().unwrap();
        let guard2 = guard1;
    }
    // regular::g();
    // swap::g();
    // option::g();
    // vector::g();
    Ok(())
}
