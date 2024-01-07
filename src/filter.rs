
pub const UNIQUE_VALUES: usize = 10;

pub fn order_lut(slice: &[u8]) -> [Vec<usize>; UNIQUE_VALUES] {
    let mut lut: [Vec<usize>; UNIQUE_VALUES] =
        count_lut(slice).map(|count| Vec::with_capacity(count as usize));
    for (i, num) in slice.iter().enumerate() {
        lut[*num as usize].push(i);
    }
    lut
}

pub fn count_lut(slice: &[u8]) -> [usize; UNIQUE_VALUES] {
    let mut lut = [0usize; UNIQUE_VALUES];
    for num in slice.iter() {
        lut[*num as usize] += 1;
    }
    return lut;
}

pub fn filter_non_occuring(slice: &[u8], other: &[u8]) -> Vec<u8> {
    let other_count_lut = count_lut(other);
    slice
        .iter()
        .filter(|item| other_count_lut[**item as usize] != 0)
        .copied()
        .collect()
}



#[cfg(test)]
mod filter_tests {
    use super::{count_lut, filter_non_occuring, order_lut};

    #[test]
    fn test_count_lut() {
        let slice = [8, 5, 1, 0, 2, 9, 8, 3, 7, 5, 6, 8, 3, 6, 3];
        dbg!(count_lut(&slice));
    }

    #[test]
    fn test_order_lut() {
        let slice = [0, 0, 0, 1, 1, 2, 3];
        dbg!(order_lut(&slice));
    }

    #[test]
    fn test_filter_non_occuring() {
        //let slice = [1, 6, 9, 7, 2, 6, 2, 4, 2, 3, 3, 8, 9, 4, 8, 9, 7, 6, 8, 5, 7, 6, 0, 7, 3, 4, 4];
        //let other = [8, 5, 1, 0, 2, 9, 8, 3, 7, 5, 6, 8, 3, 6, 3, 5, 1, 4, 0, 7, 4, 1, 9, 5, 7, 5, 8];
        let slice = [1, 6, 9, 7, 2, 6, 2, 4, 2, 3, 3, 8, 9, 4, 8];
        let other = [8, 5, 1, 0, 2, 9, 8, 3, 7, 5, 6, 8, 3, 6, 3];
        dbg!(filter_non_occuring(&slice, &other));
        dbg!(filter_non_occuring(&other, &slice));
    }
}
