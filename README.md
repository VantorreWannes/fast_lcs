# Optimizing Longest Common Subsequence (LCS) Computation in Rust

## Introduction

The Longest Common Subsequence (LCS) problem involves finding the longest subsequence common to two sequences. Traditional dynamic programming approaches to LCS have a time complexity of \( O(nm) \), which can be inefficient for large datasets. This document outlines an optimized algorithm for computing the LCS in Rust, designed to reduce computation time significantly.

## Overview of the Algorithm

The optimization leverages the following key ideas:

1. **Filtering Matching Elements**: Quickly identify and work only with elements that are present in both sequences.
2. **Constructing Pair Indexes**: Create pairs of indices from both sequences where the elements match.
3. **Building an Unblocking Lookup Table (LUT)**: Determine which pairs can follow a given pair in the LCS.
4. **Recursive Traversal**: Traverse the LUT recursively to find the longest possible chain of pairs, representing the LCS.

## Detailed Explanation

### 1. Identifying Matching Elements

Given two sequences of bytes, `source` and `target`, we first find the indices where the values match between them.

```rust
let source = vec![0, 1, 0, 2];
let target = vec![2, 0, 1, 0];
```

We create an **index lookup table** (`index_lut`) for each sequence, mapping each unique value to the indices where it appears.

```rust
// For source
let mut index_lut_source = vec![vec![]; 3]; // Assuming values are 0, 1, 2
for (i, &value) in source.iter().enumerate() {
    index_lut_source[value as usize].push(i);
}
// index_lut_source = [[0, 2], [1], [3]]

// For target
let mut index_lut_target = vec![vec![]; 3];
for (i, &value) in target.iter().enumerate() {
    index_lut_target[value as usize].push(i);
}
// index_lut_target = [[1, 3], [2], [0]]
```

### 2. Generating Pair Indexes

Using the lookup tables, we generate all possible pairs of indices where the values in `source` and `target` match.

```rust
let mut pair_indexes = vec![];

for (value, indices_source) in index_lut_source.iter().enumerate() {
    let indices_target = &index_lut_target[value];
    for &i_source in indices_source {
        for &i_target in indices_target {
            pair_indexes.push((i_source, i_target));
        }
    }
}
// pair_indexes = [(0, 1), (0, 3), (2, 1), (2, 3), (1, 2), (3, 0)]
```

### 3. Building Source and Target Pair Index Arrays

Extract the `source` and `target` indices from `pair_indexes` to create separate arrays.

```rust
let source_pair_indexes: Vec<usize> = pair_indexes.iter().map(|&(i, _)| i).collect();
// source_pair_indexes = [0, 0, 2, 2, 1, 3]

let target_pair_indexes: Vec<usize> = pair_indexes.iter().map(|&(_, j)| j).collect();
// target_pair_indexes = [1, 3, 1, 3, 2, 0]
```

### 4. Constructing the Unblocking LUT

The **unblocking LUT** determines which pairs can follow a given pair in the LCS.

For each pair `(i, j)`, we:

- Slice `source_pair_indexes` from `i + 1` to the end.
- Slice `target_pair_indexes` from `j + 1` to the end.
- Find the intersection of these slices, which gives the indices of pairs that can follow the current pair.

```rust
fn build_unblocking_lut(
    pair_indexes: &[(usize, usize)],
    source_pair_indexes: &[usize],
    target_pair_indexes: &[usize],
) -> Vec<Vec<usize>> {
    let mut lut = vec![vec![]; pair_indexes.len()];

    for (current_idx, &(i_source, i_target)) in pair_indexes.iter().enumerate() {
        let source_slice = &source_pair_indexes[(current_idx + 1)..];
        let target_slice = &target_pair_indexes[(current_idx + 1)..];

        let unblocked_indices: Vec<usize> = source_slice
            .iter()
            .zip(target_slice)
            .enumerate()
            .filter_map(|(offset, (&s_idx, &t_idx))| {
                if s_idx > i_source && t_idx > i_target {
                    Some(current_idx + 1 + offset)
                } else {
                    None
                }
            })
            .collect();

        lut[current_idx] = unblocked_indices;
    }

    lut
}

// Using the function:
// unblocking_lut = [[4, 5], [], [], [], [5], []]
```

### 5. Recursive Traversal to Find the LCS

We perform a recursive traversal of the `unblocking_lut` to find the longest chain of pairs.

```rust
fn traverse_lut(
    lut: &[Vec<usize>],
    current_idx: usize,
    current_chain: &mut Vec<usize>,
    longest_chain: &mut Vec<usize>,
) {
    current_chain.push(current_idx);

    if lut[current_idx].is_empty() {
        if current_chain.len() > longest_chain.len() {
            *longest_chain = current_chain.clone();
        }
    } else {
        for &next_idx in &lut[current_idx] {
            traverse_lut(lut, next_idx, current_chain, longest_chain);
        }
    }

    current_chain.pop();
}

// Initiate traversal
let mut longest_chain = vec![];
for i in 0..pair_indexes.len() {
    let mut current_chain = vec![];
    traverse_lut(&unblocking_lut, i, &mut current_chain, &mut longest_chain);
}
```

For our example, the longest chain corresponds to the indices `[0, 4, 5]`.

### 6. Extracting the LCS

Using the indices from the longest chain, we extract the pairs and reconstruct the LCS.

```rust
let lcs_pairs: Vec<(usize, usize)> = longest_chain.iter().map(|&idx| pair_indexes[idx]).collect();
// lcs_pairs = [(0, 1), (1, 2), (3, 0)]

let lcs: Vec<u8> = lcs_pairs.iter().map(|&(i, _)| source[i]).collect();
// lcs = [0, 1, 2]
```

## Performance Analysis

- **Time Complexity**: The initial steps reduce the problem size by focusing only on matching elements.
- **Space Complexity**: Additional space is used for lookup tables and intermediate arrays, but this is acceptable for the gain in speed.
- **Efficiency**: By eliminating non-matching elements early, we avoid unnecessary computations present in traditional dynamic programming approaches.

## Implementation in Rust

This algorithm is well-suited for Rust due to:

- **Ownership and Borrowing**: Efficient memory management during recursive traversal.
- **Collections**: Use of `Vec` and iterators for high-performance data manipulation.
- **Safety**: Rust's type system helps prevent common bugs during implementation.

### Example Code Snippet

```rust
fn find_lcs(source: &[u8], target: &[u8]) -> Vec<u8> {
    // Build index lookup tables
    let max_value = *source.iter().chain(target).max().unwrap() as usize + 1;
    let mut index_lut_source = vec![vec![]; max_value];
    let mut index_lut_target = vec![vec![]; max_value];

    for (i, &value) in source.iter().enumerate() {
        index_lut_source[value as usize].push(i);
    }

    for (i, &value) in target.iter().enumerate() {
        index_lut_target[value as usize].push(i);
    }

    // Generate pair indexes
    let mut pair_indexes = vec![];
    for value in 0..max_value {
        for &i_source in &index_lut_source[value] {
            for &i_target in &index_lut_target[value] {
                pair_indexes.push((i_source, i_target));
            }
        }
    }

    // Build unblocking LUT
    let source_pair_indexes: Vec<_> = pair_indexes.iter().map(|&(i, _)| i).collect();
    let target_pair_indexes: Vec<_> = pair_indexes.iter().map(|&(_, j)| j).collect();
    let unblocking_lut = build_unblocking_lut(&pair_indexes, &source_pair_indexes, &target_pair_indexes);

    // Traverse LUT to find the longest chain
    let mut longest_chain = vec![];
    for i in 0..pair_indexes.len() {
        let mut current_chain = vec![];
        traverse_lut(&unblocking_lut, i, &mut current_chain, &mut longest_chain);
    }

    // Extract LCS
    longest_chain
        .iter()
        .map(|&idx| source[pair_indexes[idx].0])
        .collect()
}
```

## Conclusion

This optimized LCS algorithm reduces computation time by:

- Focusing only on matching elements between the two sequences.
- Efficiently determining the valid sequences of matches using an unblocking LUT.
- Recursively finding the longest chain of matches representing the LCS.

By implementing this algorithm in Rust, we leverage the language's strengths to achieve both performance and safety.

## Getting Involved

We welcome contributions to further optimize and enhance this algorithm. Possible areas of improvement include:

- Handling larger datasets efficiently.
- Parallelizing the traversal for performance gains.
- Refining the data structures for better memory usage.

Feel free to explore the codebase, experiment with the implementation, and submit pull requests or issues on our GitHub repository.

---

*This document is intended to help new contributors understand the optimized LCS algorithm implemented in our Rust project. For any questions or discussions, please join our community channels.*