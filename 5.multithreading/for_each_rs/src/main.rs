use rayon::prelude::*;

fn main() {
    let mut x = 0;
    (0..1_000_000).into_par_iter().for_each(|_| {
        x += 1;
    });
}
