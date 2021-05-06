fn push_int_twice(v: &mut Vec<i32>, n: &i32) {
    v.push(*n);
    v.push(*n);
}

// fn push_int_twice(v: &mut Vec<i32>, n: &i32) {
// }

fn main() {
    let mut my_vector = vec![0];
    let my_int_reference = &my_vector[0];
    push_int_twice(&mut my_vector, my_int_reference);
}
