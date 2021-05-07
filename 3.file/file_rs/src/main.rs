#![allow(unused_variables)]

use std::error::Error;
use std::fs::File;

fn file_close_at_end_of_scope() -> Result<(), Box<dyn Error>> {
    let my_file = File::open("/dev/null")?;
    Ok(())
}

fn file_close_scope() -> Result<(), Box<dyn Error>> {
    {
        let my_file = File::open("/dev/null")?;
    }
    Ok(())
}

fn file_close_explicitly() -> Result<(), Box<dyn Error>> {
    let my_file = File::open("/dev/null")?;
    drop(my_file);
    Ok(())
}

fn file_close_vec_cant_drop() -> Result<(), Box<dyn Error>> {
    let my_files = vec![
        File::open("/dev/null")?,
        File::open("/dev/null")?,
        File::open("/dev/null")?,
    ];
    drop(my_files[1]);
    Ok(())
}

fn file_close_vec_remove() -> Result<(), Box<dyn Error>> {
    let mut my_files = vec![
        File::open("/dev/null")?,
        File::open("/dev/null")?,
        File::open("/dev/null")?,
    ];
    my_files.remove(1);
    Ok(())
}

fn file_close_field_drop() -> Result<(), Box<dyn Error>> {
    struct Foo {
        file: File,
    }
    let my_foo = Foo {
        file: File::open("/dev/null")?,
    };
    drop(my_foo.file);
    Ok(())
}

fn file_close_field_drop_fails() -> Result<(), Box<dyn Error>> {
    struct Foo {
        file: File,
    }
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropping!");
        }
    }
    let my_foo = Foo {
        file: File::open("/dev/null")?,
    };
    drop(my_foo.file);
    Ok(())
}

fn file_close_field_drop_option() -> Result<(), Box<dyn Error>> {
    struct Foo {
        file: Option<File>,
    }
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("dropping!");
        }
    }
    let mut my_foo = Foo {
        file: Some(File::open("/dev/null")?),
    };
    my_foo.file = None;
    Ok(())
}

fn main() {}
