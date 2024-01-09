use crate::{new_algos::{counts_lcs::Alcs, closest_offset_sum_lcs::closest_sum_offset_lcs}, slow_lcs::Lcs};
mod filter;
mod slow_lcs;
mod new_algos;

pub type MaxInputLengthType = u16;
pub const UNIQUE_VALUES: usize = 256;

fn main() {
    println!("Hello, world!");
    let source = [9, 8, 2, 8, 8, 7, 9, 9, 8, 8, 8, 1, 9, 6, 1, 5, 4, 2, 6, 2, 3, 6, 5, 5, 5, 1, 6, 6, 4, 9, 0, 1, 2, 6, 1, 7, 6, 7, 6, 6, 5, 7, 1, 1, 5, 5, 2, 0, 9, 1, 0, 7, 0, 4, 0, 3, 8, 3, 1, 6, 5, 5, 2, 6, 1, 0, 9, 8, 0, 8, 5, 8, 3, 8, 3, 0, 4, 2, 5, 3, 3, 0, 4, 2, 3, 4, 4, 1, 4, 4, 8, 7, 3, 2, 4, 3, 8, 3, 6, 5, 3, 0, 4, 1, 6, 3, 7, 5, 9, 2, 2, 6, 3, 2, 5, 6, 0, 7, 4, 3, 8, 2, 6, 3, 4, 3, 3, 5, 6, 6, 7, 6, 8, 0, 6, 4, 5, 4, 5, 2, 1, 7, 4, 5, 6, 9, 8, 0, 3, 5, 9, 6, 7, 1, 5, 4, 8, 6, 0, 9, 6, 7, 8, 4, 2, 0, 5, 9, 2, 3, 9, 8, 4, 7, 7, 9, 7, 8, 2, 7, 6, 8, 9, 2, 8, 5, 4, 8, 2, 6, 4, 2, 3, 7, 1, 1, 3, 2, 8, 6, 2, 9, 3, 3, 3, 5, 9, 9, 9, 6, 1, 7, 4, 8, 8, 8, 9, 8, 8, 8, 6, 5, 6, 3, 0, 6, 4, 5, 7, 8, 1, 4, 5, 0, 5, 6, 7, 8, 8, 6, 7, 8, 5, 7, 6, 7, 3, 6, 1, 7, 3, 0, 9, 6, 2];
    let target = [0, 3, 0, 7, 0, 8, 4, 0, 4, 4, 0, 0, 6, 2, 4, 0, 0, 5, 8, 9, 9, 2, 4, 7, 3, 6, 1, 2, 4, 2, 2, 7, 4, 8, 0, 9, 1, 3, 4, 9, 3, 3, 5, 3, 4, 2, 8, 7, 7, 9, 4, 0, 8, 2, 0, 4, 3, 7, 5, 8, 0, 9, 6, 0, 1, 2, 0, 7, 8, 9, 7, 3, 9, 7, 1, 4, 8, 4, 8, 4, 3, 7, 8, 2, 4, 3, 6, 1, 0, 4, 3, 2, 4, 6, 0, 9, 7, 0, 2, 5, 5, 2, 3, 3, 7, 1, 8, 6, 9, 0, 2, 2, 2, 3, 1, 6, 9, 8, 6, 6, 5, 3, 2, 6, 3, 7, 1, 5, 0, 7, 7, 3, 8, 6, 1, 2, 0, 2, 5, 9, 9, 7, 2, 0, 5, 6, 2, 2, 7, 4, 9, 5, 8, 9, 8, 3, 1, 0, 2, 4, 6, 6, 8, 6, 7, 7, 5, 1, 5, 6, 8, 5, 8, 2, 9, 3, 8, 2, 9, 7, 1, 9, 2, 2, 0, 2, 1, 7, 9, 6, 9, 9, 1, 1, 7, 3, 3, 9, 5, 0, 5, 7, 8, 7, 6, 4, 0, 0, 1, 9, 5, 3, 6, 2, 7, 6, 4, 0, 1, 1, 3, 7, 1, 5, 8, 7, 1, 8, 1, 9, 7, 6, 6, 9, 1, 2, 2, 4, 5, 1, 1, 1, 1, 2, 6, 5, 2, 2, 6, 7, 2, 9, 5, 0, 0];
    dbg!(Lcs::new(&source, &target).len());
    dbg!(Alcs::new(&source, &target).len());
    dbg!(closest_sum_offset_lcs(&source, &target).len());
}

#[cfg(test)]
mod tests {
    use rand::distributions::{Uniform, Distribution};
    use super::*;

    #[test]
    fn test_all() {
        let mut rng = rand::thread_rng();
        let die: Uniform<u8> = Uniform::from(0..=255);
        let source: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
        let target: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
        dbg!(Lcs::new(&source, &target).len());
        dbg!(Alcs::new(&source, &target).len());
        dbg!(closest_sum_offset_lcs(&source, &target).len());
    }

}
