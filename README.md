
## Basic functional version; Iterator output without Box.

The `basic` version contains a boxed iterator *(requiring a memory
allocation)*, because the size of the iterator is not known at
compilation time, as the return type needs to be one of multiple
iterator types. It's possible to solve this problem by returning only
one statically known type, and then using `impl Iterator` to return
the iterator. Instead of returning a boxed empty iterator, we can
return an iterator over an empty vector `Vec::new()` which doesn't
initially allocate any memory.

It's also useful to note that you can iterate through slices mutably,
and then step through multiples of said indices. Using iterators
usually means the compiler will be able to optimize away
bounds-checking without the use of unsafe.

In order to allow for inlining between crates `#[inline]` is
used, and this also can skew benchmark results significantly, so for
purposes of benchmarking, every function was tested with it.

```rust
#[inline]
pub fn primes(n: usize) -> impl Iterator<Item = usize> {
    const START: usize = 2;
    if n < START {
        Vec::new()
    } else {
        let mut is_prime = vec![true; n + 1 - START];
        let limit = (n as f64).sqrt() as usize;
        for i in START..limit + 1 {
            let mut it = is_prime[i - START..].iter_mut().step_by(i);
            if let Some(true) = it.next() {
                it.for_each(|x| *x = false);
            }
        }
        is_prime
    }
    .into_iter()
    .enumerate()
    .filter_map(|(e, b)| if b { Some(e + START) } else { None })
}
```

## Benchmarks

Beyond being more expressive the basic and bitpacked versions its
also faster then both versions on small inputs.

Benchmark results:

| n         | basic | bitpacked | functional |
|-----------|-------|-----------|------------|
| 0         | 4.3ns | 4.3ns     | 785ps      |
| 2         | 53ns  | 26ns      | 22ns       |
| 100       | 408ns | 306ns     | 215ns      |
| 1,000     | 3.6us | 2.7us     | 1.7us      |
| 10,000    | 42us  | 25us      | 16us       |
| 100,000   | 400us | 275us     | 279us      |
| 1,000,000 |       |           |            |

Benching was done with
[crieterion.rs](https://github.com/bheisler/criterion.rs):

```rust
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
        (100_000, 180, 0_800),
    ];

    // let tests = &tests[..];
    // let tests = &tests[0..3];
    // let tests = &tests[3..4];
    let tests = &tests[4..];

    let mut group = c.benchmark_group("primes");
    for &(n, time, samples) in tests {
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
```
