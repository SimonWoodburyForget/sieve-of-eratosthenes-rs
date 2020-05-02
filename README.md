
## Basic functional version; Iterator output without Box.

The `basic` version contains a boxed iterator *(requiring a memory allocation)*, because the size of the iterator is not known at compilation time, as the return type needs to be one of multiple iterator types. It's possible to solve this problem by returning only one statically known type, and then using `impl Iterator` to return the iterator. Instead of returning a boxed empty iterator, we can return an iterator over an empty vector `Vec::new()` which doesn't initially allocate any memory.

It's also useful to note that you can iterate through slices mutably, and then step through multiples of said indices. Using iterators usually means the compiler will be able to optimize away bounds-checking without the use of unsafe.

```rust
fn functional_sieve(n: usize) -> impl Iterator<Item = usize> {
    const OFFSET: usize = 2;
    if n < OFFSET {
        Vec::new()
    } else {
        let sieve = vec![true; n + 1 - OFFSET];
        let limit = (n as f64).sqrt() as usize;
        (OFFSET..limit + 1).fold(sieve, |mut sieve, i| {
            let mut it = sieve[i - OFFSET..].iter_mut().step_by(i);
            if let Some(true) = it.next() {
                it.for_each(|p| *p = false);
            }
            sieve
        })
    }
    .into_iter()
    .enumerate()
    .filter_map(|(e, b)| if b { Some(e + OFFSET) } else { None })
}
```

Benching was done with crieterion:
https://github.com/bheisler/criterion.rs, source can be found in
[`benches/main.rs`](benchs/main.rs).

Benchmark results:

| n       | basic | bitpacked | functional |
|---------|-------|-----------|------------|
| 0       | 4.3ns | 4.3ns     | 785ps      |
| 2       | 53ns  | 26ns      | 22ns       |
| 100     | 408ns | 306ns     | 215ns      |
| 1,000   | 3.6us | 2.7us     | 1.7us      |
| 10,000  | 35us  | 25us      | 17us       |
| 100,000 | 432us |           | 334us      |
