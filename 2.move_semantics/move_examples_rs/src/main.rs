#![allow(unused_variables)]

use std::fs::File;
use std::sync::Mutex;

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