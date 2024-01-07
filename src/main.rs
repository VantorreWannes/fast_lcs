use crate::{new_algos::closest_offset_sum_lcs::closest_sum_offset_lcs, slow_lcs::Lcs};
mod filter;
mod slow_lcs;

mod new_algos;

fn main() {
    println!("Hello, world!");
    let source = [1, 6, 9, 7, 2, 6, 2, 4, 2, 3, 3, 8, 9, 4, 8, 9, 7, 6, 8, 5, 7, 6, 0, 7, 3, 4, 4];
    let target = [8, 5, 1, 0, 2, 9, 8, 3, 7, 5, 6, 8, 3, 6, 3, 5, 1, 4, 0, 7, 4, 1, 9, 5, 7, 5, 8];
    dbg!(Lcs::new(&source, &target).subsequence());
    dbg!(closest_sum_offset_lcs(&source, &target));
}
