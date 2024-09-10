use crate::lcs_trait::Lcs;

#[derive(Debug, Clone, PartialEq)]
pub struct SlowLcs<'a> {
    source: &'a [u8],
    target: &'a [u8],
    table: Vec<Vec<usize>>,
}

impl<'a> SlowLcs<'a> {
    ///Max length for Source and target is `u8::MAX`!
    pub fn new(source: &'a [u8], target: &'a [u8]) -> Self {
        let table = Self::create_table(source, target);
        Self {
            table,
            source,
            target,
        }
    }

    fn create_table(source: &'a [u8], target: &'a [u8]) -> Vec<Vec<usize>> {
        let source_length = source.len();
        let target_length = target.len();
        let mut table: Vec<Vec<usize>> = vec![vec![0; target_length + 1]; source_length + 1];

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
        table
    }
}

impl Lcs for SlowLcs<'_> {
    fn subsequence(&self) -> Vec<u8> {
        let mut index = self.len();
        let mut subsequence: Vec<u8> = vec![0; index + 1];

        let mut x = self.source.len();
        let mut y = self.target.len();
        while x > 0 && y > 0 {
            if self.source[x - 1] == self.target[y - 1] {
                subsequence[index - 1] = self.source[x - 1];
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

    fn len(&self) -> usize {
        let source_length = self.source.len();
        let target_length = self.target.len();
        self.table[source_length][target_length]
    }
}

#[cfg(test)]
mod lcs_tests {
    use std::vec;

    use super::*;

    #[test]
    fn is_empty() {
        let source = vec![0; 10];
        let target = vec![];
        let lcs = SlowLcs::new(&source, &target);
        assert_eq!(lcs.len(), 0);
    }

    #[test]
    fn len() {
        let length = 10;
        let source = vec![0; length];
        let target = source.clone();
        let lcs = SlowLcs::new(&source, &target);
        assert_eq!(lcs.len(), length);
    }

    #[test]
    fn subsequence() {
        let source = b"AAA";
        let target = b"";
        let lcs = SlowLcs::new(source, target);
        assert_eq!(lcs.subsequence(), b"");

        let source = b"AAA";
        let target = &source.clone();
        let lcs = SlowLcs::new(source, target);
        assert_eq!(lcs.subsequence(), source);

        let source = b"XMJYAUZ";
        let target = b"MZJAWXU";
        let lcs = SlowLcs::new(source, target);
        assert_eq!(lcs.subsequence(), b"MJAU");
    }
}
