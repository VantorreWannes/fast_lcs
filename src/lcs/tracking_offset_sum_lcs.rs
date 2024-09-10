///!Intellectual Property of Wannes Vantorre. Distribution not permitted.
use crate::lcs_trait::Lcs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TrackingOffsetSum<'a> {
    source: &'a [u8],
    target: &'a [u8],
}

impl<'a> TrackingOffsetSum<'a> {
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        Self { source, target }
    }

    fn all_smallest_pair_offsets(source: &[u8], target: &[u8]) -> Vec<(usize, usize)> {
        let mut next_pairs_offsets = vec![];
        let mut min_offset_sum = usize::MAX;

        for (source_offset, &source_num) in source.iter().enumerate() {
            if let Some(target_offset) = target
                .iter()
                .position(|&target_num| target_num == source_num)
            {
                let offset_sum = source_offset + target_offset;

                if offset_sum < min_offset_sum {
                    next_pairs_offsets.clear();
                    next_pairs_offsets.push((source_offset, target_offset));
                    min_offset_sum = offset_sum;
                } else if offset_sum == min_offset_sum {
                    next_pairs_offsets.push((source_offset, target_offset));
                }

                if source_offset > min_offset_sum {
                    break;
                }
            }
        }
        next_pairs_offsets
    }

    fn longest_pair_offset_chain(source: &'a [u8], target: &'a [u8]) -> Vec<u8> {
        Self::all_smallest_pair_offsets(source, target)
            .iter()
            .map(|&(source_offset, target_offset)| {
                let sub_chain = Self::longest_pair_offset_chain(
                    &source[source_offset + 1..],
                    &target[target_offset + 1..],
                );
                (sub_chain.len(), sub_chain, source[source_offset])
            })
            .max_by_key(|&(len, _, _)| len)
            .map(|(_, mut chain, item)| {
                chain.insert(0, item);
                chain
            })
            .unwrap_or_else(Vec::new)
    }
}
impl Lcs for TrackingOffsetSum<'_> {
    fn subsequence(&self) -> Vec<u8> {
        Self::longest_pair_offset_chain(self.source, self.target)
    }
}

#[cfg(test)]
mod tracking_offset_sum_tests {
    use super::*;

    #[test]
    fn is_empty() {
        let source = vec![0; 10];
        let target = vec![];
        let lcs = TrackingOffsetSum::new(&source, &target);
        assert_eq!(lcs.len(), 0);
    }

    #[test]
    fn len() {
        let length = 10;
        let source = vec![0; length];
        let target = source.clone();
        let lcs = TrackingOffsetSum::new(&source, &target);
        assert_eq!(lcs.len(), length);
    }

    #[test]
    fn subsequence() {
        let source = [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3];
        let target = [3, 3, 3, 1, 1, 1, 2, 2, 2];
        assert_eq!(
            TrackingOffsetSum::new(&source, &target).subsequence(),
            vec![1, 1, 1, 2, 2, 2]
        );
    }
}
