use criterion::*;
use sieve_of_eratosthenes_rs::*;
use std::time::Duration;

fn bench_primes(c: &mut Criterion) {
    #[rustfmt::skip]
    let tests = [
        (0,       1,   2_500),
        (2,       1,   2_000),
        (100,     5,   1_800),
        (1_000,   10,  1_400),
        (10_000,  30,  1_000),
        (100_000, 200, 0_800),
    ];

    let small = &tests[0..3];
    let medium = &tests[3..4];
    let large = &tests[4..];

    let mut group = c.benchmark_group("primes");
    for &(n, time, samples) in small {
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(time));

        group.bench_with_input(BenchmarkId::new("basic::primes", n), &n, |b, &n| {
            b.iter(|| basic::primes(n).any(|x| x == 0))
        });
        group.bench_with_input(BenchmarkId::new("bitpacked::primes", n), &n, |b, &n| {
            b.iter(|| bitpacked::primes(n).any(|x| x == 0))
        });
        group.bench_with_input(BenchmarkId::new("functional::primes", n), &n, |b, &n| {
            b.iter(|| functional::primes(n).any(|x| x == 0))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_primes);
criterion_main!(benches);
