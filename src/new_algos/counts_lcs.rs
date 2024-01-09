use crate::filter::{count_lut, UNIQUE_VALUES};

pub type MaxInputLengthType = u8;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Alcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    source_counts: [MaxInputLengthType; UNIQUE_VALUES],
    target_counts: [MaxInputLengthType; UNIQUE_VALUES],
}

impl<'a> Alcs<'a> {
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Alcs {
            source,
            target,
            source_counts: Alcs::count_lut(&source),
            target_counts: Alcs::count_lut(&target),
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

    fn next_pair_offsets(source: &'a [u8], target: &'a [u8]) -> Option<(usize, usize)> {
        let source_len = source.len();
        let target_len = target.len();
        let pair_offsets = (source_len, target_len);
        todo!();
    }

    fn damage_done_by_pair(counts: &[u8]) -> f32 {
        let sum = counts.iter().map(|num| *num as usize).sum::<usize>();
        ((counts.len() as f32 / sum as f32) * 100.0).round()
    }

    fn count_lut(slice: &[u8]) -> [MaxInputLengthType; UNIQUE_VALUES] {
        let mut lut: [MaxInputLengthType; UNIQUE_VALUES] = [0; UNIQUE_VALUES];
        for num in slice.iter() {
            lut[*num as usize] += 1;
        }
        return lut;
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
