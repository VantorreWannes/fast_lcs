use crate::{lcs_trait::Lcs, MaxInputLengthType};

pub struct TreeClosestOffsetSum<'a> {
    source: &'a [u8],
    target: &'a [u8],
    pair_indexes: Vec<Vec<(MaxInputLengthType, MaxInputLengthType)>>,
}

impl<'a> TreeClosestOffsetSum<'a> {
    fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Self {
            source,
            target,
            pair_indexes: vec![],
        }
    }

    fn get_next_possible_pairs(&mut self) -> Vec<(MaxInputLengthType, MaxInputLengthType)> {
        todo!();
    }

    pub fn closest_pair_sum_offsets(source: &[u8], target: &[u8]) -> Option<(usize, usize)> {
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

impl Lcs for TreeClosestOffsetSum<'_> {
    type Item = u8;

    fn subsequence(self) -> Vec<Self::Item> {
        todo!()
    }
}
