pub mod csr_matrix;

use divan::Bencher;
use fast_lcs::utilities::{counts, filter_shared, indexes};
use rand::{distributions::Uniform, prelude::Distribution};

#[divan::bench]
fn counts_u8(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let slice: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    bencher.bench_local(move || counts(&slice));
}

#[divan::bench]
fn indexes_u8(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let slice: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    bencher.bench_local(move || indexes(&slice));
}

#[divan::bench]
fn filter_shared_usize(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<usize> = Uniform::from(0..=10000);
    let slice: Vec<usize> = die.sample_iter(&mut rng).take(10000).collect();
    let other: Vec<usize> = die.sample_iter(&mut rng).take(10000).collect();
    bencher.bench_local(move || filter_shared(&slice, &other));
}
