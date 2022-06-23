# parallel_primes
Trying parallel programming with implementing primes. From the simplest
implementation to 32 parallel threads the time for 3 million primes went down from
400 ms to 10 ms. Speedup with the same algorith (memoization) was ~8x (3 million primes) and 13x (30 million primes)
on a 16 core machine with 32 threads.
