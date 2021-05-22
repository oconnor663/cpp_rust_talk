#![allow(unused_variables)]

use std::fs::File;
use std::sync::Mutex;

fn f(s1: &mut String) {
    let s2 = *s1;
}

fn g() {
    let mut s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
    f(&mut s1);
    dbg!(s1);
}

mod swap {
    use std::mem;

    fn f(s1: &mut String) {
        let mut s2 = "".to_string();
        mem::swap(s1, &mut s2);
    }

    fn g() {
        let mut s1 = "foo".to_string();
        f(&mut s1);
    }
}

mod option {
    fn f(s1: &mut Option<String>) {
        let s2 = s1.take();
    }

    fn g() {
        let mut s1: Option<String> =
            Some("foo".to_string());
        f(&mut s1);
    }
}

mod vector {
    fn f(v: &mut Vec<String>) {
        let s2 = v.remove(0);
    }

    fn g() {
        let mut v = vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
        ];
        f(&mut v);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
        let s2 = s1;
        let v = vec![s2];
        // dbg!(s1);
        // dbg!(s2);
        dbg!(v);
    }
    {
        let s1 = "abcdefghijklmnopqrstuvwxyz".to_string();
        let s2 = s1.clone();
        let v = vec![s2.clone()];
        dbg!(s1);
        dbg!(s2);
        dbg!(v);
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
        let v = vec![s2];
        let s3 = v[0];
    }
    {
        let s1 = "abc".to_string();
        let view = s1.as_str();
        let s2 = s1;
        dbg!(view);
    }
    {
        let file1 = File::open("/dev/null")?;
        let file2 = file1;
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
    Ok(())
}
