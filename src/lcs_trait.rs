pub trait Lcs {
    fn subsequence(&self) -> Vec<u8>;

    fn len(&self) -> usize
    where
        Self: Sized,
    {
        self.subsequence().len()
    }

    fn is_empty(&self) -> bool
    where
        Self: Sized,
    {
        self.len() == 0
    }
}
