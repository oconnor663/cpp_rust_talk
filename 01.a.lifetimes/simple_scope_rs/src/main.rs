fn main() {
    let my_int_reference;
    {
        let my_int = 5;
        my_int_reference = &my_int;
    }
    dbg!(my_int_reference);
}
