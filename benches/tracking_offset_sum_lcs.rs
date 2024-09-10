use divan::{black_box, Bencher};
use fast_lcs::{lcs::tracking_offset_sum_lcs::TrackingOffsetSum, lcs_trait::Lcs};
use rand::distributions::{Distribution, Uniform};

#[divan::bench]
fn new(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    let source = black_box(&source);
    let target = black_box(&target);
    bencher.bench_local(move || TrackingOffsetSum::new(source, target));
}

#[divan::bench]
fn length(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    let lcs = TrackingOffsetSum::new(black_box(&source), black_box(&target));
    bencher.bench_local(move || lcs.len());
}

#[divan::bench]
fn subsequence(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    let lcs = TrackingOffsetSum::new(black_box(&source), black_box(&target));
    bencher.bench_local(move || lcs.subsequence());
}
