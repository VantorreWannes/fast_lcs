pub const UNIQUE_VALUES: usize = 10;

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

    fn next_pair_offsets(&self, source: &'a [u8], target: &'a [u8]) -> Option<(usize, usize)> {
        let source_len = source.len();
        let target_len = target.len();
        let mut pair_offsets = (source_len, target_len);
        let mut pair_scores = f32::INFINITY;
        for (source_offset, source_num) in source.iter().enumerate() {
            if let Some(target_offset) = target
                .iter()
                .position(|target_num| target_num == source_num)
            {
                let source_counts: Vec<MaxInputLengthType> = source[..=source_offset]
                    .iter()
                    .map(|source_num| self.source_counts[*source_num as usize])
                    .collect();
                let target_counts: Vec<MaxInputLengthType> = target[..=target_offset]
                    .iter()
                    .map(|target_num| self.target_counts[*target_num as usize])
                    .collect();
                let new_pair_scores =
                    Alcs::pair_counts_score(&source_counts) + Alcs::pair_counts_score(&target_counts);
                if new_pair_scores < pair_scores {
                    pair_offsets = (source_offset, target_offset);
                    pair_scores = new_pair_scores;
                }
            }
        }
        if pair_scores.is_infinite() {
            return None;
        }
        Some(pair_offsets)
    }

    fn pair_counts_score(counts: &[u8]) -> f32 {
        let pair_damage = Alcs::pair_counts_damage(counts);
        (pair_damage / 100.0) * counts.len() as f32
    }

    fn pair_counts_damage(counts: &[u8]) -> f32 {
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
    fn test_pair_damage() {
        let source = [4, 3, 4, 3];
        dbg!(Alcs::pair_counts_damage(&source));
    }

    #[test]
    fn test_pair_score() {
        let source = [4, 3, 4, 3];
        dbg!(Alcs::pair_counts_score(&source));
    }

    #[test]
    fn test_next_pair_offsets() {
        let source = [6, 9, 7, 2, 6, 2, 4, 2, 3, 3, 8, 9, 4, 8, 9, 7, 6, 8, 5, 7, 6, 0, 7, 3, 4, 4];
        let target = [0, 2, 9, 8, 3, 7, 5, 6, 8, 3, 6, 3, 5, 1, 4, 0, 7, 4, 1, 9, 5, 7, 5, 8];
        dbg!(Alcs::new(&source, &target).next_pair_offsets(&source, &target));
    }
}
