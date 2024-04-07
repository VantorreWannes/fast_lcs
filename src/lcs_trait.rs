pub trait Lcs<T> {
    fn subsequence(self) -> Vec<T>;

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
