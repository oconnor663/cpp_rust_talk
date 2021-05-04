fn main() {
    let mut my_int = 5;
    let reference1 = &mut my_int;
    let reference2 = &mut my_int;
    *reference1 += 1;
    *reference2 += 1;
}
