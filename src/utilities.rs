const UNIQUE_VALUES: usize =  u8::MAX as usize + 1;

pub fn indexes(slice: &[u8]) -> [Vec<usize>; UNIQUE_VALUES] {
    let mut lut = counts(slice).map(|count| Vec::<usize>::with_capacity(count as usize));
    for (i, num) in slice.iter().enumerate() {
        lut[*num as usize].push(i);
    }
    lut
}

pub fn counts(slice: &[u8]) -> [usize; UNIQUE_VALUES] {
    let mut lut = [0; UNIQUE_VALUES];
    for num in slice.iter() {
        lut[*num as usize] += 1;
    }
    return lut;
}

pub fn filter_shared(slice: &[u8], other: &[u8]) -> Vec<u8> {
    let other_count_lut = counts(other);
    slice
        .iter()
        .filter(|item| other_count_lut[**item as usize] != 0)
        .copied()
        .collect()
}

#[cfg(test)]
mod filter_tests {

    use super::{counts, filter_shared, indexes};

    #[test]
    fn test_counts() {
        let slice = vec![3, 0, 1, 2, 1, 2];
        assert_eq!(counts(&slice)[..4], [1, 2, 2, 1]);
    }

    #[test]
    fn test_indexes() {
        let slice = [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3];
        let result = [
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![9, 10],
            vec![],
        ];
        assert_eq!(indexes(&slice)[..5], result);
    }

    #[test]
    fn test_filter_shared() {
        let slice = [1, 2, 3];
        let other = [3, 0, 1, 0, 2];
        assert_eq!(
            filter_shared(&slice, &other),
            [1, 2, 3]
        );
    }
}
