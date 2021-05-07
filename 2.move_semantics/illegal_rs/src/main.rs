#![allow(unused_variables)]

fn move_int_twice() {
    let a = 1;
    let b = a;
    let c = a;
}

fn move_int_while_borrowed() {
    let a = 1;
    let b = &a;
    let c = a;
    dbg!(b);
}

fn move_int_through_reference() {
    let a = 1;
    let b = &a;
    let c = *b;
}

fn move_vec_twice() {
    let a = vec![1, 2, 3];
    let b = a;
    let c = a;
}

fn move_vec_while_borrowed() {
    let a = vec![1, 2, 3];
    let b = &a;
    let c = a;
    dbg!(b);
}

fn move_vec_through_reference() {
    let mut a = vec![1, 2, 3];
    let b = &mut a;
    let c = *b;
}

fn main() {
    move_int_twice();
    move_int_while_borrowed();
    move_int_through_reference();
    move_vec_twice();
    move_vec_while_borrowed();
    move_vec_through_reference();
}
