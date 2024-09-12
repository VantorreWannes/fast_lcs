pub mod csr_matrix;

use std::ops::Index;

use num_traits::{cast, NumCast, PrimInt, Unsigned, Zero};

pub fn indexes<T>(slice: &[T]) -> Vec<Vec<usize>>
where
    T: PrimInt + Unsigned + Zero + Default + NumCast,
{
    let counts = counts(slice);
    let mut lut = Vec::with_capacity(counts.len());
    for count in counts {
        lut.push(Vec::with_capacity(count));
    }
    for (i, num) in slice.iter().enumerate() {
        lut[cast::<T, usize>(*num).unwrap()].push(i);
    }
    lut
}

#[inline]
pub fn counts<T>(slice: &[T]) -> Vec<usize>
where
    T: PrimInt + Unsigned + Zero + Default + NumCast + Copy,
{
    if slice.is_empty() {
        return Vec::new();
    }

    // Calculate max_value in a single pass and ensure it's a usize
    let mut max_value = cast::<T, usize>(slice[0]).unwrap_or(0);
    for &num in slice.iter() {
        let num_usize = cast::<T, usize>(num).unwrap();
        if num_usize > max_value {
            max_value = num_usize;
        }
    }

    // Allocate LUT with capacity directly
    let mut lut = vec![0; max_value + 1];

    // Use iterators for optimal iteration
    for &num in slice {
        lut[cast::<T, usize>(num).unwrap()] += 1;
    }

    lut
}

#[inline]
pub fn filter_shared<T>(slice: &[T], other: &[T]) -> Vec<T>
where
    T: PrimInt + Unsigned + Zero + Default + NumCast + Copy,
{
    if other.is_empty() || slice.is_empty() {
        return Vec::new();
    }

    let other_counts = counts(other);
    let mut result = Vec::with_capacity(slice.len());
    let max_index = other_counts.len();

    for &num in slice.iter() {
        let num_index = cast::<T, usize>(num).unwrap();
        if num_index < max_index && other_counts[num_index] > 0 {
            result.push(num);
        }
    }

    result
}



#[inline]
pub fn remove_values_from_sorted<T>(arr: &mut Vec<T>, to_remove: &[T]) where T: Ord{ 
    for value in to_remove.iter() {
        remove_single_value_from_sorted(arr, value);
    }
}

#[inline]
pub fn remove_single_value_from_sorted<T>(arr: &mut Vec<T>, to_remove: &T) where T: Ord {
    if let Some(pos) = arr.iter().position(|value| value == to_remove) {
        let len = arr.len();
        arr.swap(pos, len - 1);
        _ = arr.pop();
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GenericIter<'a, T, I> {
    items: &'a T,
    current: I,
    length: I,
}

impl<'a, T, I> GenericIter<'a, T, I>
where
    I: From<u8>,
{
    pub fn new(items: &'a T, length: I) -> GenericIter<'a, T, I> {
        GenericIter {
            items,
            current: I::from(0),
            length,
        }
    }
}

impl<'a, T, I> Iterator for GenericIter<'a, T, I>
where
    T: Index<I>,
    I: std::cmp::PartialEq + std::ops::AddAssign<I> + From<u8> + Clone + Copy,
    <T as Index<I>>::Output: 'a,
{
    type Item = &'a <T as Index<I>>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.length {
            None
        } else {
            let result: &<T as Index<I>>::Output = &self.items[self.current];
            self.current += I::from(1);
            Some(result)
        }
    }
}


#[cfg(test)]
mod utilities_tests {
    use super::*;

    #[test]
    fn test_counts() {
        let slice = [0u8, 1, 1, 1, 2, 3, 4, 4, 4];
        let lut = counts(&slice);
        assert_eq!(lut[0], 1);
        assert_eq!(lut[1], 3);
        assert_eq!(lut[2], 1);
        assert_eq!(lut[3], 1);
        assert_eq!(lut[4], 3);
    }

    #[test]
    fn test_indexes() {
        let slice = [0u8, 1, 1, 1, 2, 3, 4, 4, 4];
        let lut = indexes(&slice);
        assert_eq!(lut[0], vec![0]);
        assert_eq!(lut[1], vec![1, 2, 3]);
        assert_eq!(lut[2], vec![4]);
        assert_eq!(lut[3], vec![5]);
        assert_eq!(lut[4], vec![6, 7, 8]);
        println!("{:?}", lut);
    }

    #[test]
    fn delete_values_test() {
        let mut original = vec![1, 2, 3, 4, 5];
        let to_delete = vec![1, 3, 5];
        for &num in to_delete.iter() {
            remove_single_value_from_sorted(&mut original, &num);
        }
        assert_eq!(original, vec![5, 2, 4]);
    }
}
