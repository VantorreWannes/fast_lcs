use divan::Bencher;
use fast_lcs::utilities::csr_matrix::CsrMatrix;
use rand::{distributions::Uniform, prelude::Distribution};

#[divan::bench]
fn push_u8(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let slice: Vec<u8> = die.sample_iter(&mut rng).take(100).collect();
    let mut csr_matrix = CsrMatrix::with_capacity(1000);
    bencher.bench_local(move || {
        for _ in slice.iter() {
            csr_matrix.push(&slice);
        }
    });
}

#[divan::bench]
fn pop_u8(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let slice: Vec<u8> = die.sample_iter(&mut rng).take(100).collect();
    let mut csr_matrix = CsrMatrix::with_capacity(1000);
    for _ in slice.iter() {
        csr_matrix.push(&slice);
    }
    bencher.bench_local(move || csr_matrix.pop());
}