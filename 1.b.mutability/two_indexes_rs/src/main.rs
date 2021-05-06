use std::cell::{RefCell, RefMut};

fn f_fail() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let first_element = &mut char_array[0];
    let second_element = &mut char_array[1];
    *first_element = 'c';
    *second_element = 'd';
}

fn f_indexes() {
    let mut char_array: [char; 2] = ['a', 'b'];
    char_array[0] = 'c';
    char_array[1] = 'd';
    assert_eq!(char_array, ['c', 'd']);
}

fn f_split() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let (first_slice, rest_slice) = char_array.split_at_mut(1);
    let first_element = &mut first_slice[0];
    let second_element = &mut rest_slice[0];
    *first_element = 'c';
    *second_element = 'd';
    assert_eq!(char_array, ['c', 'd']);
}

fn f_iterator() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let mut array_iterator = char_array.iter_mut();
    let first_element = array_iterator.next().unwrap();
    let second_element = array_iterator.next().unwrap();
    *first_element = 'c';
    *second_element = 'd';
    assert_eq!(char_array, ['c', 'd']);
}

fn f_refcell() {
    let char_array: [RefCell<char>; 2] = [RefCell::new('a'), RefCell::new('b')];
    let mut first_element: RefMut<char> = char_array[0].borrow_mut();
    let mut second_element: RefMut<char> = char_array[1].borrow_mut();
    *first_element = 'c';
    *second_element = 'd';
    drop(first_element);
    drop(second_element);
    assert_eq!(char_array, [RefCell::new('c'), RefCell::new('d')]);
}

fn f_unsafe() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let first_element: *mut char = &mut char_array[0];
    let second_element: *mut char = &mut char_array[1];
    unsafe {
        *first_element = 'c';
        *second_element = 'd';
    }
    assert_eq!(char_array, ['c', 'd']);
}

fn main() {
    f_fail();
    f_indexes();
    f_split();
    f_iterator();
    f_refcell();
    f_unsafe();
}
