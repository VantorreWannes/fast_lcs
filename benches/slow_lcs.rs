use divan::{black_box, Bencher};
use fast_lcs::slow_lcs::Lcs;
use rand::distributions::{Distribution, Uniform};

#[divan::bench]
fn bench_slow_lcs(bencher: Bencher) {
    let mut rng = rand::thread_rng();
    let die: Uniform<u8> = Uniform::from(0..=255);
    let source: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    let target: Vec<u8> = die.sample_iter(&mut rng).take(1000).collect();
    bencher.bench_local(move || {
        black_box(Lcs::new(black_box(&source), black_box(&target)).subsequence());
    });
}
