fn main() {
    let my_string = "hello world".to_string();
    let mut my_vector = Vec::new();
    my_vector.push(my_string.clone());
    my_vector.push(my_string);
}
