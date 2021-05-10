fn main() {
    let my_int_ptr: &i32;
    {
        let my_int: i32 = 5;
        my_int_ptr = &my_int;
    }
    dbg!(*my_int_ptr);
}
