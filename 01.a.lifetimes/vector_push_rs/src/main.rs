fn main() {
    let mut my_vector: Vec<&str> = Vec::new();
    {
        let my_string = "hello world".to_string();
        my_vector.push(&my_string);
    }
    dbg!(my_vector);
}
