use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::utilities::{
    counts, csr_matrix::CsrMatrix, filter_shared, indexes
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PairScoreLcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    pair_indexes: Vec<(usize, usize)>,
    unblocking_lut: CsrMatrix<usize>,
}

impl<'a> PairScoreLcs<'a> {
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        let mut pair_indexes = Self::pair_indexes(source, target);
        // pair_indexes.sort_unstable();
        let unblocking_lut = Self::unblocking_lut(&pair_indexes);
        Self {
            source,
            target,
            pair_indexes,
            unblocking_lut,
        }
    }

    fn pair_indexes(source: &[u8], target: &[u8]) -> Vec<(usize, usize)> {
        let source_counts = counts(source);
        let target_counts = counts(target);
        let pair_indexes_len = source_counts
            .iter()
            .zip(target_counts.iter())
            .map(|(&source_count, &target_count)| source_count.min(target_count))
            .sum::<usize>();
        let source_indexes = indexes(source);
        let target_indexes = indexes(target);
        let mut pair_indexes = Vec::with_capacity(pair_indexes_len);
        for (source_indexes, target_indexes) in source_indexes.iter().zip(target_indexes.iter()) {
            for source_index in source_indexes {
                for target_index in target_indexes {
                    pair_indexes.push((*source_index, *target_index));
                }
            }
        }
        pair_indexes
    }

    fn pair_index_lut(pair_indexes: &[usize]) -> CsrMatrix<usize> {
        let pair_indexes_max = pair_indexes.iter().max().copied().unwrap_or_default();
        let pair_indexes_counts = counts(pair_indexes);
        let mut pair_index_lut = (0..=pair_indexes_max)
            .into_iter()
            .map(|index| {
                let capacity = pair_indexes_counts.get(index).copied().unwrap_or_default();
                Vec::with_capacity(capacity)
            })
            .collect::<Vec<_>>();
        for (index, &pair_index) in pair_indexes.iter().enumerate() {
            pair_index_lut[pair_index].push(index);
        }
        CsrMatrix::from(pair_index_lut.as_slice())
    }

    fn unblocking_lut(pair_indexes: &[(usize, usize)]) -> CsrMatrix<usize> {
        let source_pair_indexes = pair_indexes
             .iter()
             .map(|(source_indexes, _)| *source_indexes)
            .collect::<Vec<usize>>();
        let target_pair_indexes = pair_indexes
            .iter()
            .map(|(_, target_indexes)| *target_indexes)
            .collect::<Vec<usize>>();
        let source_pair_indexes_lut = Self::pair_index_lut(&source_pair_indexes);
        let target_pair_indexes_lut = Self::pair_index_lut(&target_pair_indexes);

        let unblocking_lut = pair_indexes.par_iter().map(|&(source_index, target_index)| {
            let source_pair_indexes_slice = &source_pair_indexes_lut[source_index+1..];
            let target_pair_indexes_slice = &target_pair_indexes_lut[target_index+1..];
            filter_shared(source_pair_indexes_slice, target_pair_indexes_slice)
        }).collect::<Vec<Vec<_>>>();
        CsrMatrix::from(unblocking_lut.as_slice())

        // [2, 0, 1, 0]
        // [0, 1, 0, 2]
        // [(0, 1), (0, 3), (2, 1), (2, 3), (1, 2), (3, 0)]
        // [[0, 1], [4, ?], [2, 3], [5, ?]]
        // [[5, ?], [0, 2], [4, ?], [1, 3]]
        // [[4, 3], [], [], [], [3], []]

        // // [[4, 2, 3, 5, ?], [2, 3, 5], [5, ?], []]
        // // [[0, 2, 4, 1, 3], [4, 1, 3], [1, 3], []]
        // // [[4, 3], [], [], [], [3], []]
        

        // // [[0, 1], [0, 1, 4], [0, 1, 4, 2, 3], [0, 1, 4, 2, 3, 5]]
        // // [[5, ?], [5, 0, 2], [5, 0, 2, 4, ?], [5, 0, 2, 4, 1, 3]]

    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn dgb() {
        let source = [0, 1, 0, 2];
        let target = [2, 0, 1, 0];
        let pair_indexes = PairScoreLcs::pair_indexes(&source, &target);
        println!("{:?}", pair_indexes);
    }

    #[test]
    fn pair_indexes() {
        let source = vec![0, 1, 0, 1];
        let target = vec![1, 0, 1, 0];
        let pair_indexes = PairScoreLcs::pair_indexes(&source, &target);
        assert_eq!(
            pair_indexes,
            vec![
                (0, 1),
                (0, 3),
                (2, 1),
                (2, 3),
                (1, 0),
                (1, 2),
                (3, 0),
                (3, 2)
            ]
        );
    }

    #[test]
    fn pair_index_lut() {
        let pair_indexes = [0, 1, 1, 3];
        let pair_index_lut = PairScoreLcs::pair_index_lut(&pair_indexes);
        let other_pair_index_lut =
            CsrMatrix::from(vec![vec![0], vec![1, 2], vec![], vec![3]].as_slice());
        assert_eq!(pair_index_lut, other_pair_index_lut);
    }

    #[test]
    fn unblocking_lut() {
        let pair_indexes = [(0, 0), (1, 1)];
        let unblocking_lut = PairScoreLcs::unblocking_lut(&pair_indexes);
        let other_unblocking_lut = CsrMatrix::from(vec![vec![1], vec![]].as_slice());
        assert_eq!(unblocking_lut, other_unblocking_lut);
    }

    #[test]
    fn lis_lcs() {
        
    }
}
