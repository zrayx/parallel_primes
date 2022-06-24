# parallel_primes
Trying parallel programming with implementing primes. From the simplest
implementation to 32 parallel threads the time for 3 million primes went down from
~400 ms to 9 ms. Speedup with the same algorith (memoization) was ~8x (3 million primes) and 13x (30 million primes)
on a 16 core machine with 32 threads.

opt-level = 3 in Cargo.toml, but opt-level = 2 delivered the same results.

See also my implementation of the same algorithms in Zig: https://github.com/zrayx/parallel_primes_again

3 million primes:

    P1: Time elapsed: 403, sum: 216817, max: 3000000
    P2: Time elapsed: 394, sum: 216816, max: 3000000
    P3: Time elapsed: 81, sum: 216816, max: 3000000
    P4: Time elapsed: 82, sum: 216816, max: 3000000
    P5: Time elapsed: 78, sum: 216816, max: 3000000
    P6: Time elapsed: 97, sum: 216816, max: 3000000
    P7: Time elapsed: 20, sum: 216822, max: 3000000
    P8: Time elapsed: 9, sum: 216818, threads: 16
    P8: Time elapsed: 11, sum: 216822, threads: 32

30 million primes:

    P1: Time elapsed: 10750, sum: 1857860, max: 30000000
    P2: Time elapsed: 10688, sum: 1857859, max: 30000000
    P3: Time elapsed: 1693, sum: 1857859, max: 30000000
    P4: Time elapsed: 1694, sum: 1857859, max: 30000000
    P5: Time elapsed: 1651, sum: 1857859, max: 30000000
    P6: Time elapsed: 2921, sum: 1857859, max: 30000000
    P7: Time elapsed: 187, sum: 1857866, max: 30000000
    P8: Time elapsed: 149, sum: 1857863, threads: 16
    P8: Time elapsed: 146, sum: 1857866, threads: 32
    P10: Time elapsed: 121, sum: 1857861, max: 30000000
    P11: Time elapsed: 157, sum: 1857861, max: 30000000

Notable implementations:

P1 - naive implementation
P3 - memoization
P8 - memoization, multiple threads
P10 - Sieve of Eratosthenes, single thread
P11 - Sieve of Eratosthenes, single thread, using packed bits
