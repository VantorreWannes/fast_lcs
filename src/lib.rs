pub mod filter;
pub mod new_algos;
pub mod slow_lcs;

pub type MaxInputLengthType = u8;
pub const UNIQUE_VALUES: usize = 256;

#[cfg(test)]
mod tests {
    use crate::{
        filter::{count_lut, index_lut},
        new_algos::{closest_offset_sum_lcs::closest_sum_offset_lcs, counts_lcs::Alcs},
        slow_lcs::Lcs,
    };
    use rand::distributions::{Distribution, Uniform};

    #[test]
    fn test_all() {
        let mut rng = rand::thread_rng();
        let die: Uniform<u8> = Uniform::from(0..=1);
        let source: Vec<u8> = die.sample_iter(&mut rng).take(20).collect();
        let target: Vec<u8> = die.sample_iter(&mut rng).take(20).collect();
        dbg!(Lcs::new(&source, &target).subsequence());
        //dbg!(Alcs::new(&source, &target).len());
        dbg!(closest_sum_offset_lcs(&source, &target));
        dbg!(&source, &target);
        dbg!(Lcs::new(&source, &target).len());
        dbg!(closest_sum_offset_lcs(&source, &target).len());
    }

    #[test]
    fn test_bin() {
        let source = [0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0];
        let target = [0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0];
        dbg!(Lcs::new(&source, &target).subsequence());
        dbg!(Lcs::new(&source, &target).len());
        dbg!(closest_sum_offset_lcs(&source, &target));
        dbg!(closest_sum_offset_lcs(&source, &target).len());
    }

    #[test]
    fn dbg_indexes_bin() {
        let source = [0, 0, 1, 1, 0, 1, 0];
        let target = [1, 1, 1, 0, 1, 0, 0, 1];
        dbg!(index_lut(&source));
        dbg!(index_lut(&target));
    }

    #[test]
    fn dbg_count_bin() {
        let source = [0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0];
        let target = [0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0];
        dbg!(count_lut(&source));
        dbg!(count_lut(&target));
    }
}
