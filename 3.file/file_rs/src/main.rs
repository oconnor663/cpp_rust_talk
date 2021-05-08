#![allow(unused_variables)]
#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::mem;

fn file_close() -> Result<(), Box<dyn Error>> {
    let my_file = File::open("/dev/null")?;
    dbg!(mem::size_of::<File>()); // prints 4
    // drop closes my_file
    Ok(())
}

fn file_close_scope() -> Result<(), Box<dyn Error>> {
    {
        let my_file = File::open("/dev/null")?;
    }
    Ok(())
}

fn file_close_method() {
    // no such thing
}

fn file_close_drop() -> Result<(), Box<dyn Error>> {
    let my_file = File::open("/dev/null")?;
    drop(my_file);
    Ok(())
}

fn file_close_vec() -> Result<(), Box<dyn Error>> {
    let my_files = vec![
        File::open("/dev/null")?,
        File::open("/dev/null")?,
        File::open("/dev/null")?,
    ];
    drop(my_files[1]);
    Ok(())
}

fn file_close_vec_erase() -> Result<(), Box<dyn Error>> {
    let mut my_files = vec![
        File::open("/dev/null")?,
        File::open("/dev/null")?,
        File::open("/dev/null")?,
    ];
    my_files.remove(1);
    Ok(())
}

fn file_close_field_1() -> Result<(), Box<dyn Error>> {
    struct Foo {
        file: File,
    }
    let my_foo = Foo {
        file: File::open("/dev/null")?,
    };
    drop(my_foo.file);
    Ok(())
}

fn file_close_field_2() -> Result<(), Box<dyn Error>> {
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

fn file_close_field_option() -> Result<(), Box<dyn Error>> {
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

fn main() {
    dbg!(mem::size_of::<File>());
}
