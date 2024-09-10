pub mod lcs;
pub mod lcs_trait;
pub mod utilities;

#[cfg(test)]
mod uniform_comparison_tests {
    use crate::{
        lcs::{closest_offset_sum_lcs::ClosestOffsetSum, slow_lcs::SlowLcs},
        lcs_trait::Lcs,
    };
    use rand::distributions::{Distribution, Uniform};

    #[test]
    fn compare_lcs_lengths() {
        let mut rng = rand::thread_rng();
        let die: Uniform<u8> = Uniform::from(0..=255);
        let source: Vec<u8> = die.sample_iter(&mut rng).take(2000).collect();
        let target: Vec<u8> = die.sample_iter(&mut rng).take(2000).collect();
        println!(
            "SlowLcs LCS length: {}",
            SlowLcs::new(&source, &target).len()
        );
        println!(
            "ClosestOffsetSum LCS length {}",
            ClosestOffsetSum::new(&source, &target).len()
        );
    }
}

#[cfg(test)]
mod debug_tests {
    use crate::{
        lcs::{closest_offset_sum_lcs::ClosestOffsetSum, slow_lcs::SlowLcs},
        lcs_trait::Lcs,
    };

    #[test]
    fn dbg_slow_lcs() {
        let source = [0, 0, 1, 1, 0, 1, 0];
        let target = [1, 1, 1, 0, 1, 0, 0, 1];
        println!(
            "ClosestOffsetSum LCS values {:#?}",
            SlowLcs::new(&source, &target).subsequence()
        );
        println!(
            "SlowLcs LCS length: {}",
            SlowLcs::new(&source, &target).len()
        );
    }

    #[test]
    fn dbg_closest_offset_sum_lcs() {
        let source = [0, 0, 1, 1, 0, 1, 0];
        let target = [1, 1, 1, 0, 1, 0, 0, 1];
        println!(
            "ClosestOffsetSum LCS values {:#?}",
            ClosestOffsetSum::new(&source, &target).subsequence()
        );
        println!(
            "ClosestOffsetSum LCS length {}",
            ClosestOffsetSum::new(&source, &target).len()
        );
    }
}
