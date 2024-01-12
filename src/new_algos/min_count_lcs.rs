use crate::{MaxInputLengthType, UNIQUE_VALUES};

use super::lcs_trait::Lcs;

pub struct Alcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    source_counts: [MaxInputLengthType; UNIQUE_VALUES],
    target_counts: [MaxInputLengthType; UNIQUE_VALUES],
}

impl<'a> Alcs<'a> {
    fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Self {
            source,
            target,
            source_counts: Alcs::count_lut(source),
            target_counts: Alcs::count_lut(target),
        }
    }

    fn count_lut(slice: &[u8]) -> [MaxInputLengthType; UNIQUE_VALUES] {
        let mut lut: [MaxInputLengthType; UNIQUE_VALUES] = [0; UNIQUE_VALUES];
        for num in slice.iter() {
            lut[*num as usize] += 1;
        }
        return lut;
    }
}

impl<'a> Lcs for Alcs<'a> {
    type Item = u8;
    fn subsequence(self) -> Vec<u8> {
        todo!()
    }
}
