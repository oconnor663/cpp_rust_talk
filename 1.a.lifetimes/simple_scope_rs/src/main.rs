fn main() {
    let my_int_reference: &i32;
    {
        let my_int: i32 = 5;
        my_int_reference = &my_int;
    }
    dbg!(*my_int_reference);
}
