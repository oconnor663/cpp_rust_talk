use std::cell::{Ref, RefCell, RefMut};

fn f_fail() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let first_element = &mut char_array[0];
    let second_element = &char_array[1];
    *first_element = *second_element;
}

fn f_indexes() {
    let mut char_array: [char; 2] = ['a', 'b'];
    char_array[0] = char_array[1];
    assert_eq!(char_array, ['b', 'b']);
}

fn f_split() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let (first_slice, rest_slice) = char_array.split_at_mut(1);
    let first_element = &mut first_slice[0];
    let second_element = &rest_slice[0];
    *first_element = *second_element;
    assert_eq!(char_array, ['b', 'b']);
}

fn f_iterator() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let mut array_iterator = char_array.iter_mut();
    let first_element = array_iterator.next().unwrap();
    let second_element = array_iterator.next().unwrap();
    *first_element = *second_element;
    assert_eq!(char_array, ['b', 'b']);
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn f_refcell() {
    let char_array: [RefCell<char>; 2] =
        [RefCell::new('a'), RefCell::new('b')];
    let mut first_guard: RefMut<char> =
        char_array[0].borrow_mut();
    let second_guard: Ref<char> =
        char_array[1].borrow();
    let first_element: &mut char = &mut *first_guard;
    let second_element: &char = &*second_guard;
    *first_element = *second_element;
    drop(first_guard);
    drop(second_guard);
    assert_eq!(char_array, [RefCell::new('b'), RefCell::new('b')]);
}

fn f_unsafe() {
    let mut char_array: [char; 2] = ['a', 'b'];
    let first_element: *mut char = &mut char_array[0];
    let second_element: *const char = &char_array[1];
    unsafe {
        *first_element = *second_element;
    }
    assert_eq!(char_array, ['b', 'b']);
}

fn main() {
    f_fail();
    f_indexes();
    f_split();
    f_iterator();
    f_refcell();
    f_unsafe();
}
