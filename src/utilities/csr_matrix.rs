use std::{fmt::Debug, ops::{Index, IndexMut, Range, RangeFrom, RangeInclusive}};

use super::GenericIter;

#[derive(PartialEq, Eq, Clone, Default)]
pub struct CsrMatrix<T> {
    items: Vec<T>,
    offsets: Vec<usize>,
}

impl<T> CsrMatrix<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            offsets: vec![0],
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity + 1);
        offsets.push(0);
        CsrMatrix {
            items: Vec::with_capacity(capacity),
            offsets,
        }
    }

    pub fn len(&self) -> usize {
        self.offsets.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<&[T]> {
        if self.offsets.len() < 2 || index > *self.offsets.last().unwrap() {
            return None;
        }
        Some(&self[index])
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut [T]> {
        if self.offsets.len() < 2 || index > *self.offsets.last().unwrap() {
            return None;
        }
        Some(&mut self[index])
    }

    pub fn iter(&self) -> GenericIter<'_, CsrMatrix<T>, usize> {
        GenericIter::new(self, self.len())
    }
}

impl<T> CsrMatrix<T>
where
    T: Clone,
{
    pub fn push(&mut self, item: &[T]) {
        self.items.extend_from_slice(item);
        let last_offset = self.offsets.last().copied().unwrap_or_default();
        self.offsets.push(last_offset + item.len());
    }

    pub fn pop(&mut self) -> Vec<T> {
        _ = self.offsets.pop();
        let last_offset = self.offsets.last().copied().unwrap_or_default();
        self.items.drain(last_offset..).collect()
    }

    pub fn extend_from_slice(&mut self, item: &[Vec<T>]) {
        for i in item {
            self.push(i);
        }
    }
}

impl<T> Index<usize> for CsrMatrix<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        let start = self.offsets[index];
        let end = self.offsets[index + 1];
        &self.items[start..end]
    }
}

impl<T> IndexMut<usize> for CsrMatrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = self.offsets[index];
        let end = self.offsets[index + 1];
        &mut self.items[start..end]
    }
}

impl<T> Index<Range<usize>> for CsrMatrix<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        let start = self.offsets[index.start];
        let end = self.offsets[index.end];
        &self.items[start..end]
    }
}

impl<T> IndexMut<Range<usize>> for CsrMatrix<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        let start = self.offsets[index.start];
        let end = self.offsets[index.end];
        &mut self.items[start..end]
    }
}

impl<T> Index<RangeInclusive<usize>> for CsrMatrix<T> {
    type Output = [T];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        let start = self.offsets[*index.start()];
        let end = self.offsets[*index.end() + 1];
        &self.items[start..end]
    }
}

impl<T> IndexMut<RangeInclusive<usize>> for CsrMatrix<T> {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        let start = self.offsets[*index.start()];
        let end = self.offsets[*index.end() + 1];
        &mut self.items[start..end]
    }
}


impl<T> Index<RangeFrom<usize>> for CsrMatrix<T> {
    type Output = [T];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        let start = self.offsets[index.start];
        &self.items[start..]
    }
}


impl<T> IndexMut<RangeFrom<usize>> for CsrMatrix<T> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        let start = self.offsets[index.start];
        &mut self.items[start..]
    }
}

impl<'a, T> From<&'a CsrMatrix<T>> for GenericIter<'a, CsrMatrix<T>, usize> {

    fn from(value: &'a CsrMatrix<T>) -> Self {
        GenericIter::new(value, value.len())
    }
}

impl<T> From<&[Vec<T>]> for CsrMatrix<T> where T: std::clone::Clone {
    fn from(value: &[Vec<T>]) -> Self {
        let mut matrix = CsrMatrix::new();
        matrix.extend_from_slice(value);
        matrix
    }
}

impl<T> Debug for CsrMatrix<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let combined = self.iter().collect::<Vec<&[T]>>(); 
        f.debug_struct("CsrMatrix").field("combined", &combined).field("items", &self.items).field("offsets", &self.offsets).finish()
    }
}

#[cfg(test)]
mod csr_matrix_tests {

    use super::*;

    #[test]
    fn push() {
        let mut matrix = CsrMatrix::new();
        matrix.push(&[1, 2, 3]);
        matrix.push(&[4, 5, 6]);
        assert_eq!(matrix.len(), 2);
        assert_eq!(matrix[0], [1, 2, 3]);
        assert_eq!(matrix[1], [4, 5, 6]);
    }

    #[test]
    fn pop() {
        let mut matrix = CsrMatrix::new();
        matrix.push(&[1, 2, 3]);
        matrix.push(&[4, 5, 6]);
        assert_eq!(matrix.pop(), vec![4, 5, 6]);
        assert_eq!(matrix[0], [1, 2, 3]);
    }

    #[test]
    fn extend_from_slice() {
        let mut matrix = CsrMatrix::new();
        matrix.extend_from_slice(&[vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(matrix.len(), 2);
        assert_eq!(matrix[0], [1, 2, 3]);
        assert_eq!(matrix[1], [4, 5, 6]);
    }

    #[test]
    fn index_range() {
        let mut matrix = CsrMatrix::new();
        matrix.push(&[1, 2, 3]);
        matrix.push(&[4, 5, 6]);
        assert_eq!(&matrix[0..2], &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn iter() {
        let mut matrix = CsrMatrix::<usize>::new();
        matrix.extend_from_slice(&[vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(matrix.iter().collect::<Vec<_>>(), vec![&[1, 2, 3], &[4, 5, 6]]);
    }
}