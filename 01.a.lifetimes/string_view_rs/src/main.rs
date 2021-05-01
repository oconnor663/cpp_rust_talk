fn main() {
    let my_string: String = "hello".to_string();
    let my_string_view: &str = (my_string + " world").as_str();
    dbg!(my_string_view);
}
