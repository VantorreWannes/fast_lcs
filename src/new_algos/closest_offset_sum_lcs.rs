///!Intellectual Property of Wannes Vantorre. Distribution not permitted.

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
