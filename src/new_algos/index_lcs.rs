use std::cmp::min;

use crate::filter::{count_lut, filter_non_occuring};

pub fn lcs(source: &[u8], target: &[u8]) -> Vec<usize> {
    let source = filter_non_occuring(&source, &target);
    let target = filter_non_occuring(&target, &source);
    let source_counts = count_lut(&source);
    let target_counts = count_lut(&target);
    dbg!(&source_counts, &target_counts);
    source_counts
        .iter()
        .zip(target_counts.iter())
        .map(|(source_count, target_count)| *min(source_count, target_count) as usize)
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod index_lcs_tests {
    use crate::slow_lcs::Lcs;

    use super::*;

    #[test]
    fn test_testing() {
        let source = [
            1, 6, 9, 7, 2, 6, 2, 4, 2, 3, 3, 8, 9, 4, 8, 9, 7, 6, 8, 5, 7, 6, 0, 7, 3, 4, 4,
        ];
        let target = [
            8, 5, 1, 0, 2, 9, 8, 3, 7, 5, 6, 8, 3, 6, 3, 5, 1, 4, 0, 7, 4, 1, 9, 5, 7, 5, 8,
        ];
        dbg!(Lcs::new(&source, &target).subsequence());
        dbg!(lcs(&source, &target));
    }
}
