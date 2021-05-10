fn main() {
    let my_string: String = "abcdefghijklmnopqrstuvwxy".to_string();
    let my_string_view: &str = (my_string + "z").as_str();
    dbg!(my_string_view);
}
