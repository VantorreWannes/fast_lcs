use std::cmp::min;

use crate::{lcs_trait::Lcs, MaxInputLengthType, UNIQUE_VALUES};

#[derive(Debug, Clone, PartialEq)]
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

    fn next_pair_offsets(&self, source: &[u8], target: &[u8]) -> Option<(usize, usize)> {
        let source_len = source.len();
        let target_len = target.len();
        let default_min_count_summed =
            Self::min_count_summed(&self.source_counts, &self.target_counts);
        let mut old_min_count_summed: u32 = 0;
        let mut pair_offsets = (source_len, target_len);
        for (source_offset, source_num) in source.iter().enumerate() {
            if let Some(target_offset) = target
                .iter()
                .position(|target_num| target_num == source_num)
            {
                // big bad but i'll fix it later
                let mut source_counts = self.source_counts.clone();
                let mut target_counts = self.target_counts.clone();
                for source_num in &source[..=source_offset] {
                    source_counts[*source_num as usize] -= 1;
                }
                for target_num in &target[..=target_offset] {
                    target_counts[*target_num as usize] -= 1;
                }
                let new_min_count_summed = Self::min_count_summed(&source_counts, &target_counts);
                if default_min_count_summed - new_min_count_summed
                    <= default_min_count_summed - old_min_count_summed
                {
                    old_min_count_summed = new_min_count_summed;
                    pair_offsets = (source_offset, target_offset);
                }
            }
        }
        if pair_offsets == (source_len, target_len) {
            return None;
        }
        return Some(pair_offsets);
    }

    fn min_count_summed(
        source_counts: &[MaxInputLengthType; UNIQUE_VALUES],
        target_counts: &[MaxInputLengthType; UNIQUE_VALUES],
    ) -> u32 {
        let min_count = source_counts
            .iter()
            .zip(target_counts.iter())
            .map(|(source_num, target_num)| min(source_num, target_num))
            .copied()
            .collect::<Vec<MaxInputLengthType>>();
        min_count.iter().map(|num| *num as u32).sum::<u32>()
    }
}

impl<'a> Lcs for Alcs<'a> {
    type Item = u8;
    fn subsequence(mut self) -> Vec<Self::Item> {
        let mut last_lcs_indexes: (usize, usize) = (0, 0);
        let mut lcs: Vec<u8> = vec![];
        while let Some((source_offset, target_offset)) = self.next_pair_offsets(
            &self.source[last_lcs_indexes.0..],
            &self.target[last_lcs_indexes.1..],
        ) {
            for source_num in &self.source[last_lcs_indexes.0..=last_lcs_indexes.0 + source_offset]
            {
                self.source_counts[*source_num as usize] -= 1;
            }
            for target_num in &self.target[last_lcs_indexes.1..=last_lcs_indexes.1 + target_offset]
            {
                self.target_counts[*target_num as usize] -= 1;
            }
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
mod min_counts_tests {
    use rand::{distributions::Standard, Rng};

    use crate::{filter::filter_non_occuring, slow_lcs::SlowLcs};

    use super::*;

    #[test]
    fn test_next_pair_offsets() {
        let source = [1, 1, 1, 0, 1, 0];
        let target = [0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0];
        dbg!(Alcs::new(&source, &target).next_pair_offsets(&source, &target));
    }

    #[test]
    fn test_subsequence() {
        let source = [0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0];
        let target = [0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0];
        dbg!(Alcs::new(&source, &target).subsequence());
        dbg!(Alcs::new(&source, &target).len());

        // [0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0]
        // [0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0]
        // [0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]
    }

    #[test]
    fn test_find_broken_inputs() {
        const CHUNK_SIZE: usize = 15;
        let mut rng = rand::thread_rng().sample_iter(Standard);
        let mut source: Vec<u8> = (&mut rng).take(CHUNK_SIZE).collect();
        let mut target: Vec<u8> = (&mut rng).take(CHUNK_SIZE).collect();
        source = filter_non_occuring(&source, &target);
        target = filter_non_occuring(&target, &source);
        //let mut cso_lcs = closest_sum_offset_lcs(&source, &target);
        let mut alcs = Alcs::new(&source, &target);
        let mut slow_lcs = SlowLcs::new(&source, &target);
        while alcs.len() == slow_lcs.len() {
            source = (&mut rng).take(CHUNK_SIZE).collect();
            target = (&mut rng).take(CHUNK_SIZE).collect();
            source = filter_non_occuring(&source, &target);
            target = filter_non_occuring(&target, &source);
            //cso_lcs = closest_sum_offset_lcs(&source, &target);
            alcs = Alcs::new(&source, &target);
            slow_lcs = SlowLcs::new(&source, &target);
        }
        println!("{:?}", &source);
        println!("{:?}", &target);
        println!("{:?}", Alcs::new(&source, &target).subsequence());
        println!("{:?}", SlowLcs::new(&source, &target).subsequence());
    }
}
