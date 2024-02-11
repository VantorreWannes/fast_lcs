///!Intellectual Property of Wannes Vantorre. Distribution not permitted.

///Intellectual Property of Wannes Vantorre. Distribution not permitted.
pub fn closest_sum_offset_lcs(source: &[u8], target: &[u8]) -> Vec<u8> {
    let mut last_lcs_indexes: (usize, usize) = (0, 0);
    let mut lcs: Vec<u8> = vec![];
    while let Some((source_offset, target_offset)) =
        closest_pair_sum_offsets(&source[last_lcs_indexes.0..], &target[last_lcs_indexes.1..])
    {
        last_lcs_indexes = (
            last_lcs_indexes.0 + source_offset + 1,
            last_lcs_indexes.1 + target_offset + 1,
        );
        lcs.push(source[last_lcs_indexes.0 - 1]);
    }
    lcs
}

///Intellectual Property of Wannes Vantorre. Distribution not permitted.
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

#[cfg(test)]
mod closest_offset_sum_tests {
    use rand::{distributions::Standard, Rng};

    use crate::{filter::filter_non_occuring, new_algos::counts_lcs::Alcs, slow_lcs::SlowLcs};

    use super::*;

    #[test]
    fn test_closest_pair_sum_offsets() {
        let source = [4, 161, 255, 161];
        let target = [161, 4, 161, 255];
        println!("{:?}", closest_sum_offset_lcs(&source, &target).len());
        println!("{:?}", SlowLcs::new(&source, &target).len());
    }

    #[test]
    fn test_alcs() {
        let source = [132, 28];
        let target = [28, 132, 28];
        println!("{:?}", Alcs::new(&source, &target).len());
        println!("{:?}", SlowLcs::new(&source, &target).len());
    }

    #[test]
    fn test_find_broken_inputs() {
        const CHUNK_SIZE: usize = 15;
        let mut rng = rand::thread_rng().sample_iter(Standard);
        let mut source: Vec<u8> = (&mut rng).take(CHUNK_SIZE).collect();
        let mut target: Vec<u8> = (&mut rng).take(CHUNK_SIZE).collect();
        source = filter_non_occuring(&source, &target);
        target = filter_non_occuring(&target, &source);
        let mut cso_lcs = closest_sum_offset_lcs(&source, &target);
        let mut slow_lcs = SlowLcs::new(&source, &target);
        while cso_lcs.len() == slow_lcs.len() {
            source = (&mut rng).take(CHUNK_SIZE).collect();
            target = (&mut rng).take(CHUNK_SIZE).collect();
            source = filter_non_occuring(&source, &target);
            target = filter_non_occuring(&target, &source);
            cso_lcs = closest_sum_offset_lcs(&source, &target);
            slow_lcs = SlowLcs::new(&source, &target);
        }
        println!("{:?}", &source);
        println!("{:?}", &target);
        println!("{:?}", closest_sum_offset_lcs(&source, &target));
        println!("{:?}", SlowLcs::new(&source, &target).subsequence());
    }
}
