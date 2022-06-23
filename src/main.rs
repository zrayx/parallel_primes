use std::sync::Arc;

use std::thread;

fn is_prime1(n: u64) -> u64 {
    if n <= 3 {
        return 1;
    }
    let mut i: u64 = 2;
    while i * i <= n {
        if n % i == 0 {
            return 0;
        }
        i += 1;
    }

    1
}

fn p1(max: u64) {
    let time_start = std::time::SystemTime::now();

    let mut sum = 0;
    for i in 1..max {
        sum += is_prime1(i);
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P1: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

fn p2(max: u64) {
    let time_start = std::time::SystemTime::now();

    let mut i = 5;
    let mut sum = 2; // two primes below 5
    while i < max {
        sum += is_prime1(i);
        sum += is_prime1(i + 2);
        i += 4;
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P2: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

struct Primes3 {
    primes: Vec<u64>,
}

impl Primes3 {
    pub fn new() -> Primes3 {
        Primes3 {
            primes: vec![2, 3, 5, 7, 11, 13],
        }
    }

    // has to be called monotically with every possible prime, no gaps
    // specifically, all primes from 0..sqrt(n) have to be memoized
    // e.g. from 0..max
    pub fn to_prime_mut(&mut self, n: u64) -> u64 {
        if n <= 1 {
            return 0;
        };
        if n > 1 && n <= 3 {
            self.primes.push(n);
            return 1;
        }

        for i in 0.. {
            let p = self.primes[i];
            if p * p > n {
                break;
            }
            if n % p == 0 {
                return 0;
            }
        }
        self.primes.push(n);

        1
    }

    // check if the number is a prime and store it in v if yes
    // requires that all primes up to sqrt(start) are already stored in self.primes
    fn is_prime_store(&self, n: u64, v: &mut Vec<u64>) {
        if n <= 1 {
            return;
        };
        if n > 1 && n <= 3 {
            v.push(n);
        }

        for i in 0.. {
            let p = self.primes[i];
            if p * p > n {
                break;
            }
            if n % p == 0 {
                return;
            }
        }
        v.push(n);
    }

    // return a slice of primes between start and end (excl.)
    // requires that all primes up to sqrt(start) are already stored in self.primes
    pub fn prime_slice(&self, start: u64, end: u64) -> Vec<u64> {
        let mut v = vec![];
        let mut i = start;
        while i < end {
            self.is_prime_store(i, &mut v);
            i += 2;
        }
        v
    }

    // append a slice to the internal store. meant to be called after prime_slice()
    pub fn prime_slice_store(&mut self, v: &mut Vec<u64>) {
        self.primes.append(v);
    }
}

// memoization
fn p3(max: u64) {
    let time_start = std::time::SystemTime::now();

    let mut i = 5;
    let mut sum = 2; // two primes below 5
    let mut p = Primes3::new();
    while i < max {
        sum += p.to_prime_mut(i);
        i += 2;
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P3: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

// memoization, step size = 4
fn p4(max: u64) {
    let time_start = std::time::SystemTime::now();

    let mut i = 5;
    let mut sum = 2; // two primes below 5
    let mut p = Primes3::new();
    while i < max {
        sum += p.to_prime_mut(i);
        sum += p.to_prime_mut(i + 2);
        i += 4;
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P4: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

// memoization in slices, preparing for multi threading
fn p5(max: u64) {
    let time_start = std::time::SystemTime::now();

    let mut sum = 6; // six primes below 15
    let mut p = Primes3::new();
    const STEP_SIZE: u64 = 3000;
    const THREADS: u64 = 4;
    let mut last = 15; // primes up to 13 are already stored

    while last < max {
        let step_to_max = (max - last) / THREADS;
        let step_root = (last * last - last) / THREADS;
        let step = step_to_max.min(step_root).min(STEP_SIZE).max(4) & 0xfffffffffffffffe;
        let mut v = p.prime_slice(last, last + step);
        last += step;
        sum += v.len();
        p.prime_slice_store(&mut v);
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P5: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

fn is_prime_store(n: u64, small_primes: &[u64], v: &mut Vec<u64>) {
    if n <= 1 {
        return;
    };
    if n > 1 && n <= 3 {
        v.push(n);
    }

    for p in small_primes {
        if p * p > n {
            break;
        }
        if n % p == 0 {
            return;
        }
    }
    v.push(n);
}

fn prime_slice(small_primes: &[u64], start: u64, end: u64) -> Vec<u64> {
    let mut v = vec![];
    let mut i = start;
    while i < end {
        is_prime_store(i, small_primes, &mut v);
        i += 2;
    }
    v
}

// memoization in 1 thread
fn p6(max: u64) {
    let time_start = std::time::SystemTime::now();

    let mut sum = 6; // six primes below 15
    const STEP_SIZE: u64 = 50_000;
    const THREADS: u64 = 4;
    let mut last = 15; // primes up to 13 are already stored

    // RwLock
    let mut primes = vec![2, 3, 5, 7, 11, 13];

    while last < max {
        let step_to_max = (max - last) / THREADS;
        let step_root = (last * last - last) / THREADS;
        let step = step_to_max.min(step_root).min(STEP_SIZE).max(4) & 0xfffffffffffffffe;
        let start = last;
        let end = last + step;
        let read_primes = primes.clone();
        let mut result = thread::spawn(move || prime_slice(&read_primes, start, end))
            .join()
            .unwrap();

        last += step;
        sum += result.len();
        primes.append(&mut result);
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P6: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

// memoization in threads
fn p7(max: u64) {
    let time_start = std::time::SystemTime::now();

    let mut sum = 6; // six primes below 15
    const STEP_SIZE: u64 = 500_000;
    const THREADS: u64 = 32;
    let mut last = 15; // primes up to 13 are already stored

    // RwLock
    let mut primes = vec![2, 3, 5, 7, 11, 13];

    while last < max {
        let step_to_max = (max - last) / THREADS;
        let step_root = (last * last - last) / THREADS;
        let step = step_to_max.min(step_root).min(STEP_SIZE).max(4) & 0xfffffffffffffffe;
        let mut threads = vec![];
        for i in 0..THREADS {
            let start = last + i * step;
            let end = last + (i + 1) * step;
            let read_primes = primes.clone();
            let rs = thread::spawn(move || prime_slice(&read_primes, start, end));
            threads.push(rs);
        }

        for t in threads {
            let mut result = t.join().unwrap();
            sum += result.len();
            primes.append(&mut result);
        }

        last += step * THREADS;
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P7: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

// copy data less frequently
fn p8(max: u64) {
    for step_size in [/*50_000, 200_000, 1_000_000, 5_000_000,*/ 50_000_000] {
        for thread_count in [16, 32] {
            let time_start = std::time::SystemTime::now();
            let mut sum = 6; // six primes below 15
            let mut last = 15; // primes up to 13 are already stored

            // RwLock
            let mut primes = vec![2, 3, 5, 7, 11, 13];

            while last < max {
                let step_to_max = (max - last) / thread_count;
                let step_root = (last * last - last) / thread_count;
                let step = step_to_max.min(step_root).min(step_size).max(4) & 0xfffffffffffffffe;
                let mut threads = vec![];
                let read_primes = Arc::new(primes.clone());
                for i in 0..thread_count {
                    let start = last + i * step;
                    let end = last + (i + 1) * step;
                    let clone = read_primes.clone();
                    let rs = thread::spawn(move || prime_slice(&clone, start, end));
                    threads.push(rs);
                }

                for t in threads {
                    let mut result = t.join().unwrap();
                    sum += result.len();
                    primes.append(&mut result);
                }

                last += step * thread_count;
            }

            let time_elapsed = time_start.elapsed().unwrap().as_millis();
            println!(
                "P8: Time elapsed: {}, sum: {}, threads: {}",
                time_elapsed, sum, thread_count
            );
        }
    }
}

fn main() {
    let num = 30_000_000;
    p1(num);
    p2(num);
    p3(num);
    p4(num);
    p5(num);
    p6(num);
    p7(num);
    p8(num);

    // let v = vec![1, 2, 3];

    // let val = Arc::new(v);

    // let mut threads = vec![];
    // for _ in 0..10 {
    //     let val = Arc::clone(&val);

    //     threads.push(thread::spawn(move || {
    //         println!("{}", val.len());
    //     }));
    // }
    // for t in threads {
    //     t.join();
    // }
    // let mut v = Arc::try_unwrap(val).unwrap();
    // v.push(1);
    // println!("final: {}", v.len());
}
