use fast_lcs::lcs_trait::Lcs;

pub type MaxLcsLength = u16;

#[derive(Debug, PartialEq, Clone)]
pub struct SlowLcs<'a, T> {
    source: &'a [T],
    target: &'a [T],
    table: Vec<Vec<MaxLcsLength>>,
}

impl<'a, T> SlowLcs<'a, T>
where
    T: PartialEq,
{
    pub fn new(source: &'a [T], target: &'a [T]) -> Self {
        let source_length = source.len();
        let target_length = target.len();
        assert!(source_length <= MaxLcsLength::MAX as usize);
        assert!(target_length <= MaxLcsLength::MAX as usize);
        let mut table: Vec<Vec<MaxLcsLength>> = vec![vec![0; target_length + 1]; source_length + 1];
        for x in 0..=source_length {
            for y in 0..=target_length {
                if x == 0 || y == 0 {
                    table[x][y] = 0
                } else if source[x - 1] == target[y - 1] {
                    table[x][y] = table[x - 1][y - 1] + 1
                } else {
                    table[x][y] = table[x - 1][y].max(table[x][y - 1])
                }
            }
        }
        Self {
            table,
            source,
            target,
        }
    }
}

impl<T> Lcs<T> for SlowLcs<'_, T>
where
    T: PartialEq + Default + Copy,
{
    fn subsequence(self) -> Vec<T> {
        let mut x = self.source.len();
        let mut y = self.target.len();
        let mut index = self.table[x][y];
        let mut subsequence: Vec<T> = vec![T::default(); index as usize + 1];
        while x > 0 && y > 0 {
            if self.source[x - 1] == self.target[y - 1] {
                subsequence[index as usize - 1] = self.source[x - 1];
                x -= 1;
                y -= 1;
                index -= 1
            } else if self.table[x - 1][y] > self.table[x][y - 1] {
                x -= 1
            } else {
                y -= 1
            }
        }

        subsequence.pop();
        subsequence
    }
}

mod naive_tests {
    use super::*;

    #[test]
    fn naive_lcs() {
        let source = "hello world".chars().collect::<Vec<_>>();
        let target = "Hi there; hello!".chars().collect::<Vec<_>>();
        let lcs = SlowLcs::new(&source, &target);
        let subsequence = lcs.subsequence().iter().collect::<String>();
        assert_eq!(&subsequence, "hello");
    }
}
