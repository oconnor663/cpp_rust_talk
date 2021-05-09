use rayon::prelude::*;

fn main() {
    let mut x = 0;
    (0..=5).into_par_iter().for_each(|i| {
        x += i;
    });
    dbg!(x);
}
