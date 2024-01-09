use crate::filter::{count_lut, UNIQUE_VALUES};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Alcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    source_counts: [u8; UNIQUE_VALUES],
    target_counts: [u8; UNIQUE_VALUES],
}

impl<'a> Alcs<'a> {
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Alcs {
            source,
            target,
            source_counts: count_lut(&source),
            target_counts: count_lut(&target),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.subsequence().len()
    }

    pub fn subsequence(&self) -> Vec<u8> {
        todo!();
    }

    fn next_pair_offsets(&self) -> Option<(usize, usize)> {
        todo!();
    }

    fn damage_done_by_pair(counts: &[u8]) -> f32 {
        let sum = counts.iter().map(|num| *num as usize).sum::<usize>();
        ((counts.len() as f32 / sum as f32) * 100.0).round()
    }
}

#[cfg(test)]
mod compare_counts_tests {
    use super::*;

    #[test]
    fn test_damage_done_percent() {
        dbg!(Alcs::damage_done_by_pair(&[0, 1]));
    }
}
