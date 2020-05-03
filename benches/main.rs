use criterion::*;
use sieve_of_eratosthenes_rs::*;
use std::time::Duration;

macro_rules! bench_primes {
    ($g:expr, $n:expr => {$($name:expr, $f:expr;)*}) => {
        $(
            let name = format!("{}", $name);
            $g.bench_with_input(BenchmarkId::new(name, $n), &$n, |b, &n| {
                b.iter(|| $f(n).any(|x| x == 0))
            });
        )*
    };
}

fn bench_primes(c: &mut Criterion) {
    #[rustfmt::skip]
    let tests = [
        (0,         1,   2_500),
        (2,         1,   2_000),
        (100,       5,   1_800),
        (1_000,     10,  1_400),
        (10_000,    30,  1_000),
        (100_000,   180, 800),
        (1_000_000, 250, 150),
    ];

    let mut group = c.benchmark_group("primes");
    for &(n, time, samples) in tests.iter() {
        group
            .sample_size(samples)
            .measurement_time(Duration::from_secs(time));

        bench_primes!(group, n => {
            "functional", functional::primes;
            "basic", basic::primes;
            "bitpacked", bitpacked::primes;
        });
    }
    group.finish();
}

criterion_group!(benches, bench_primes);
criterion_main!(benches);
