use divan::{black_box, Bencher};
use fast_lcs::new_algos::counts_lcs::Alcs;
use rand::distributions::{Distribution, Uniform};

#[divan::bench]
fn bench_counts_lcs(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    bencher.bench_local(move || {
        black_box(Alcs::new(black_box(&source), black_box(&target)).subsequence());
    });
}
