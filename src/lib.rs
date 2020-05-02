pub mod functional {
    #[inline(always)]
    pub fn primes(n: usize) -> impl Iterator<Item = usize> {
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
}

pub mod basic {
    use std::iter::empty;

    #[inline(always)]
    pub fn primes(limit: usize) -> Box<dyn Iterator<Item = usize>> {
        if limit < 2 {
            return Box::new(empty());
        }

        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        if limit >= 1 {
            is_prime[1] = false
        }

        let sqrtlmt = (limit as f64).sqrt() as usize + 1;
        for num in 2..sqrtlmt {
            if is_prime[num] {
                let mut multiple = num * num;
                while multiple <= limit {
                    is_prime[multiple] = false;
                    multiple += num;
                }
            }
        }

        Box::new(
            is_prime
                .into_iter()
                .enumerate()
                .filter_map(|(p, is_prm)| if is_prm { Some(p) } else { None }),
        )
    }
}

pub mod bitpacked {
    use std::iter::{empty, once};

    #[inline(always)]
    pub fn primes(limit: usize) -> Box<dyn Iterator<Item = usize>> {
        if limit < 3 {
            return if limit < 2 {
                Box::new(empty())
            } else {
                Box::new(once(2))
            };
        }

        let ndxlmt = (limit - 3) / 2 + 1;
        let bfsz = ((limit - 3) / 2) / 32 + 1;
        let mut cmpsts = vec![0u32; bfsz];
        let sqrtndxlmt = ((limit as f64).sqrt() as usize - 3) / 2 + 1;

        for ndx in 0..sqrtndxlmt {
            if (cmpsts[ndx >> 5] & (1u32 << (ndx & 31))) == 0 {
                let p = ndx + ndx + 3;
                let mut cullpos = (p * p - 3) / 2;
                while cullpos < ndxlmt {
                    unsafe {
                        // avoids array bounds check, which is already done above
                        let cptr = cmpsts.get_unchecked_mut(cullpos >> 5);
                        *cptr |= 1u32 << (cullpos & 31);
                    }
                    //                cmpsts[cullpos >> 5] |= 1u32 << (cullpos & 31); // with bounds check
                    cullpos += p;
                }
            }
        }

        Box::new((-1..ndxlmt as isize).into_iter().filter_map(move |i| {
            if i < 0 {
                Some(2)
            } else {
                if cmpsts[i as usize >> 5] & (1u32 << (i & 31)) == 0 {
                    Some((i + i + 3) as usize)
                } else {
                    None
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod offbyone {
        use super::*;

        fn checks(f: impl Fn(usize) -> Vec<usize>) {
            let ps: Vec<usize> = f(100);
            assert_eq!(&ps[..5], &[2, 3, 5, 7, 11]);
            let ps: Vec<usize> = f(100);
            assert_eq!(&ps[..5], &[2, 3, 5, 7, 11]);
            assert_eq!(ps[24], 97);
            let ps: Vec<usize> = f(0);
            assert_eq!(&ps, &[]);
            let ps: Vec<usize> = f(1);
            assert_eq!(&ps, &[]);
            let ps: Vec<usize> = f(2);
            assert_eq!(&ps, &[2]);
            let ps: Vec<usize> = f(3);
            assert_eq!(&ps, &[2, 3]);
        }

        #[test]
        fn functional() {
            checks(|n| functional::primes(n).collect());
        }

        #[test]
        fn basic() {
            checks(|n| basic::primes(n).collect());
        }

        #[test]
        #[ignore]
        fn bitpacked() {
            todo!("FIXME: bitpacked fails on small primes.");
            // checks(|n| bitpacked::primes(n).collect());
        }
    }

    mod dataset {
        use super::*;

        #[test]
        #[ignore]
        fn tiny() {
            todo!("FIXME: bitpacked fails on small primes.");
            // checks(0..10);
        }

        #[test]
        fn small() {
            checks(10..100);
        }

        #[test]
        fn medium() {
            checks(100..2_000);
        }

        #[test]
        #[ignore]
        fn large() {
            checks(2_000..10_000);
        }

        fn checks(range: std::ops::Range<usize>) {
            let f = |n| {
                assert_eq!(functional::primes(n).last(), Some(n));
                assert_eq!(basic::primes(n).last(), Some(n));
                assert_eq!(bitpacked::primes(n).last(), Some(n));
            };

            let nf = |n| {
                assert_ne!(functional::primes(n).last(), Some(n));
                assert_ne!(basic::primes(n).last(), Some(n));
                assert_ne!(bitpacked::primes(n).last(), Some(n));
            };

            f(7901);
            f(4013);
            f(4013);
            f(4409);
            nf(0);
            nf(1);
            nf(10);
            nf(10_000);

            let mut primes: Vec<usize> = include_str!("primes.txt")
                .trim()
                .split("\n")
                .filter(|p| !p.starts_with("#"))
                .filter(|p| !p.trim().is_empty())
                .map(|p| p.parse().unwrap())
                .collect();
            primes.sort();

            for i in range {
                if primes.binary_search(&i).is_ok() {
                    f(i);
                } else {
                    nf(i);
                }
            }
        }
    }
}
