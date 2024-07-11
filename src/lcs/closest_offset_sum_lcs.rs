///!Intellectual Property of Wannes Vantorre. Distribution not permitted.

use crate::lcs_trait::Lcs;

pub struct ClosestOffsetSum<'a> {
    source: &'a [u8],
    target: &'a [u8],
}

impl<'a> ClosestOffsetSum<'a> {
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Self { source, target }
    }

    fn closest_pair_sum_offsets(source: &[u8], target: &[u8]) -> Option<(usize, usize)> {
        let source_len = source.len();
        let target_len = target.len();
        let mut closest_offsets: (usize, usize) = (source_len, target_len);
        for (source_offset, source_num) in source.iter().enumerate() {
            if source_offset > closest_offsets.0 + closest_offsets.1 {
                return Some((closest_offsets.0, closest_offsets.1));
            }
            if let Some(target_offset) = target
                .iter()
                .position(|target_num| target_num == source_num)
            {
                if source_offset + target_offset <= closest_offsets.0 + closest_offsets.1 {
                    closest_offsets = (source_offset, target_offset);
                }
            }
        }

        if closest_offsets.0 == source_len && closest_offsets.1 == target_len {
            return None;
        }
        Some(closest_offsets)
    }
}

impl Lcs for ClosestOffsetSum<'_> {
   
    fn subsequence(&self) -> Vec<u8> {
        let mut last_lcs_indexes: (usize, usize) = (0, 0);
        let mut lcs: Vec<u8> = vec![];
        while let Some((source_offset, target_offset)) =
            ClosestOffsetSum::closest_pair_sum_offsets(&self.source[last_lcs_indexes.0..], &self.target[last_lcs_indexes.1..])
        {
            last_lcs_indexes = (
                last_lcs_indexes.0 + source_offset + 1,
                last_lcs_indexes.1 + target_offset + 1,
            );
            lcs.push(self.source[last_lcs_indexes.0 - 1]);
        }
        lcs
    }
}

#[cfg(test)]
mod closest_offset_sum_tests {
       use super::*;

       #[test]
       fn is_empty() {
           let source = vec![0; 10];
           let target = vec![];
           let lcs = ClosestOffsetSum::new(&source, &target);
           assert_eq!(lcs.len(), 0);
       }
   
       #[test]
       fn len() {
           let length = 10;
           let source = vec![0; length];
           let target = source.clone();
           let lcs = ClosestOffsetSum::new(&source, &target);
           assert_eq!(lcs.len(), length);
       }

    #[test]
    fn subsequence() {
        let source = [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3];
        let target = [3, 3, 3, 1, 1, 1, 2, 2, 2];
        assert_eq!(ClosestOffsetSum::new(&source, &target).subsequence(), vec![1, 1, 1, 2, 2, 2]);
    }
}
