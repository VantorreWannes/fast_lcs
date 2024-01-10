pub mod slow_lcs;
pub mod new_algos;

pub type MaxInputLengthType = u16;
pub const UNIQUE_VALUES: usize = 256;

#[cfg(test)]
mod tests {
    use rand::distributions::{Uniform, Distribution};
    use crate::{slow_lcs::Lcs, new_algos::{counts_lcs::Alcs, closest_offset_sum_lcs::closest_sum_offset_lcs}};

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
