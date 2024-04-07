pub trait Lcs {
    type Item;
    fn subsequence(self) -> Vec<Self::Item>;

    fn len(self) -> usize
    where
        Self: Sized,
    {
        self.subsequence().len()
    }

    fn is_empty(self) -> bool
    where
        Self: Sized,
    {
        self.len() == 0
    }
}
