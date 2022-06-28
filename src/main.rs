use packed_bits::PackedBits;
use std::env;
use std::sync::Arc;
use std::thread;

mod packed_bits;

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

fn sieve(max: u64) -> Vec<bool> {
    let mut primes: Vec<bool> = vec![true; (max + 1) as usize];
    primes[0] = false;
    primes[1] = false;
    let mut first_prime: u64 = 2;
    while first_prime * first_prime <= max {
        for i in (first_prime * 2..=max).step_by(first_prime as usize) {
            primes[i as usize] = false;
        }
        first_prime += 1;
        while first_prime < max && !primes[first_prime as usize] {
            first_prime += 1;
        }
    }
    primes
}

// Sieve of Eratosthenes, single thread, Vec<bool>
fn p10(max: u64) {
    let time_start = std::time::SystemTime::now();

    let primes = sieve(max);

    let mut sum = 0;
    for p in primes {
        if p {
            sum += 1
        };
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P10: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

// Sieve of Eratosthenes, packet bools
fn p11(max: u64) {
    let time_start = std::time::SystemTime::now();

    let mut primes = PackedBits::new_set(max as usize + 1, true);
    let mut first_prime: u64 = 2;
    while first_prime * first_prime <= max {
        for i in (first_prime * 2..=max).step_by(first_prime as usize) {
            primes.clear(i as usize);
        }
        first_prime += 1;
        while first_prime < max && !primes.is_set(first_prime as usize) {
            first_prime += 1;
        }
    }

    let mut sum = 0;
    for i in 0..max {
        if primes.is_set(i as usize) {
            sum += 1
        };
    }

    let time_elapsed = time_start.elapsed().unwrap().as_millis();
    println!(
        "P11: Time elapsed: {}, sum: {}, max: {}",
        time_elapsed, sum, max
    );
}

fn recursive_primes_p12(max: usize, thread_count: usize) -> Vec<bool> {
    if max <= 100 {
        return sieve(max as u64 - 1);
    }

    let slice_size = (max + thread_count) / (thread_count + 1);
    let mut small_primes = recursive_primes_p12(slice_size, thread_count);
    while small_primes.len() > slice_size {
        let _ = small_primes.pop();
    }
    let mut threads = vec![];
    let small_primes_arc = Arc::new(small_primes);
    for thread_idx in 1..=thread_count {
        let start = thread_idx * slice_size;
        let small_primes_clone = small_primes_arc.clone();
        let result = thread::spawn(move || {
            let mut v = vec![true; slice_size];
            for (idx, p) in small_primes_clone.iter().enumerate() {
                if *p {
                    let start_multiple = (start + idx - 1) / idx * idx;
                    let idx_multiple = start_multiple - start;
                    for i in (idx_multiple..slice_size).step_by(idx) {
                        v[i] = false;
                    }
                }
            }

            v
        });
        threads.push(result);
    }
    let mut result = small_primes_arc.as_slice().to_vec();
    for thread in threads {
        let mut v = thread.join().unwrap();
        result.append(&mut v);
    }

    result
}

// Sieve, multithreaded
fn p12(max: u64) {
    for thread_count in 4..=4 {
        let time_start = std::time::SystemTime::now();
        let primes = recursive_primes_p12(max as usize, thread_count);
        let mut sum = 0;
        for p in primes {
            if p {
                sum += 1;
            };
        }

        let time_elapsed = time_start.elapsed().unwrap().as_millis();
        println!(
            "P12: Time elapsed: {}, sum: {}, max: {}M, threads: {}",
            time_elapsed,
            sum,
            max / 1_000_000,
            thread_count
        );
    }
}

fn recursive_primes_p12a(max: usize, thread_count: usize, init_size: usize) -> Vec<bool> {
    if max <= init_size {
        return sieve(max as u64 - 1);
    }

    let slice_size = (max + thread_count) / (thread_count + 1);
    let slice_size = (slice_size + init_size) / init_size * init_size;
    let mut small_primes = if slice_size * slice_size <= max {
        sieve(slice_size as u64 - 1)
    } else {
        recursive_primes_p12a(slice_size, thread_count, init_size)
    };
    while small_primes.len() > slice_size {
        let _ = small_primes.pop();
    }
    let mut threads = vec![];
    let small_primes_arc = Arc::new(small_primes);
    for thread_idx in 1..=thread_count {
        let start = thread_idx * slice_size;
        let small_primes_clone = small_primes_arc.clone();
        let result = thread::spawn(move || {
            let mut v = vec![true; slice_size];
            for (idx, p) in small_primes_clone.iter().enumerate() {
                if *p {
                    let start_multiple = (start + idx - 1) / idx * idx;
                    let idx_multiple = start_multiple - start;
                    for i in (idx_multiple..slice_size).step_by(idx) {
                        v[i] = false;
                    }
                }
            }

            v
        });
        threads.push(result);
    }
    let mut result = small_primes_arc.as_slice().to_vec();
    for thread in threads {
        let mut v = thread.join().unwrap();
        result.append(&mut v);
    }

    result
}

// Sieve, multithreaded, aligned
fn p12a(max: u64) {
    let mut init_size = 32 * 1024;
    while init_size <= 32 * 1024 {
        for thread_count in 4..=4 {
            let time_start = std::time::SystemTime::now();
            let primes = recursive_primes_p12a(max as usize, thread_count, init_size);
            let mut sum = 0;
            for p in primes {
                if p {
                    sum += 1;
                };
            }

            let time_elapsed = time_start.elapsed().unwrap().as_millis();
            println!(
                "P12a: Time elapsed: {}, sum: {}, max: {}M, threads: {}, init_size: {}",
                time_elapsed,
                sum,
                max / 1_000_000,
                thread_count,
                init_size
            );
        }
        init_size *= 2;
    }
}

fn recursive_primes_p16(max: usize, thread_count: usize, page_size: usize) -> Vec<bool> {
    if max <= page_size {
        return sieve(max as u64 - 1);
    }

    let small_primes = if page_size * page_size > max {
        sieve(page_size as u64 - 1)
    } else {
        let slice_size = (max as f64).sqrt().trunc() as usize;
        let slice_size = (slice_size + page_size) / page_size * page_size;
        recursive_primes_p16(slice_size, thread_count, page_size)
    };

    let slice_size = max / thread_count / 8;
    let slice_size = (slice_size + page_size) / page_size * page_size;
    let slice_size = if slice_size == max {
        page_size
    } else {
        slice_size
    };

    let mut start = small_primes.len();

    let small_primes_arc = Arc::new(small_primes);
    let mut result = small_primes_arc.as_slice().to_vec();
    while start < max {
        let mut threads = vec![];
        for _ in 1..=thread_count {
            let small_primes_clone = small_primes_arc.clone();
            let end = (start + slice_size).min(max);
            let result = thread::spawn(move || {
                let mut v = vec![true; end - start];
                for (idx, p) in small_primes_clone.iter().enumerate() {
                    if *p {
                        let j = (start + idx - 1) / idx * idx;
                        for i in (j..end).step_by(idx) {
                            v[i - start] = false;
                        }
                    }
                }

                v
            });
            threads.push(result);
            start = end;
        }
        for thread in threads {
            let mut v = thread.join().unwrap();
            result.append(&mut v);
        }
    }
    let _ = result.split_off(max);
    result
}

// like 12a, but better alignment
fn p16(max: u64, print_detail: bool) {
    let page_size_min = if print_detail { 512 } else { 4096 };
    let page_size_max = if print_detail { 1024 } else { 4096 };
    let mut page_size = page_size_min;
    let thread_min = if print_detail { 8 } else { 16 };
    let thread_max = if print_detail { 32 } else { 16 };
    let mut min_time = 99999999;
    let mut thread_count = thread_min;

    if print_detail {
        print!("{:7}", "thd/pg");
        while thread_count <= thread_max {
            print!("{:7}", thread_count);
            thread_count *= 2;
        }
        println!();
    }

    while page_size <= page_size_max {
        if print_detail {
            print!("{:7}", page_size);
        }
        let mut thread_count = thread_min;
        while thread_count <= thread_max {
            let time_start = std::time::SystemTime::now();
            let primes = recursive_primes_p16(max as usize, thread_count, page_size);
            let mut sum: i128 = 0;
            for p in primes {
                if p {
                    sum += 1;
                };
            }

            let time_elapsed = time_start.elapsed().unwrap().as_millis();
            if print_detail {
                if time_elapsed < min_time + min_time / 10 {
                    if time_elapsed < min_time {
                        min_time = time_elapsed;
                    }
                    print!("\x1b[93m");
                }
                print!("{:7}\x1b[0m", time_elapsed);
            }
            let check = |expected| {
                if sum != expected {
                    println!("\x1b[93msum={}, but should be ~{}\x1b[0m", sum, expected);
                }
            };
            match max {
                3_000_000_000 => check(144449537),
                300_000_000 => check(16252325),
                30_000_000 => check(1857859),
                3_000_000 => check(216816),
                300 => check(62),
                _ => {
                    println!("\x1b[93mhave {}, but sum unknown for {}\x1b[0m", sum, max);
                }
            }
            if !print_detail {
                // impossible
                println!(
                    "P16: Time elapsed: {}, sum: {}, max: {}M, threads: {}, page_size: {}",
                    time_elapsed,
                    sum,
                    max / 1_000_000,
                    thread_count,
                    page_size
                );
            }
            thread_count *= 2;
        }
        if print_detail {
            println!();
        }

        page_size *= 2;
    }
}

fn packed_sieve(max: u64) -> PackedBits {
    let mut primes = PackedBits::new_set(max as usize + 1, true);
    primes.clear(0);
    primes.clear(1);

    let mut first_prime: u64 = 2;
    while first_prime * first_prime <= max {
        for i in (first_prime * 2..=max).step_by(first_prime as usize) {
            primes.clear(i as usize);
        }
        first_prime += 1;
        while first_prime < max && !primes.is_set(first_prime as usize) {
            first_prime += 1;
        }
    }
    primes
}

fn recursive_primes_p17(max: usize, thread_count: usize, page_size: usize) -> PackedBits {
    if max <= page_size {
        return packed_sieve(max as u64 - 1);
    }

    let small_primes = if page_size * page_size > max {
        packed_sieve(page_size as u64 - 1)
    } else {
        let slice_size = (max as f64).sqrt().trunc() as usize;
        let slice_size = (slice_size + page_size) / page_size * page_size;
        recursive_primes_p17(slice_size, thread_count, page_size)
    };

    let slice_size = max / thread_count / 8;
    let slice_size = (slice_size + page_size) / page_size * page_size;
    let slice_size = if slice_size == max {
        page_size
    } else {
        slice_size
    };

    let mut start = small_primes.len();

    let mut result = small_primes.clone();
    //let small_primes_arc = Arc::new(small_primes);
    while start < max {
        let mut threads = vec![];
        for _ in 1..=thread_count {
            let small_primes_clone = small_primes.clone();
            let end = (start + slice_size).min(max);
            let result = thread::spawn(move || {
                let mut v = PackedBits::new_set(end - start, true);
                for idx in 0..small_primes_clone.len() {
                    if small_primes_clone.is_set(idx) {
                        let j = (start + idx - 1) / idx * idx;
                        for i in (j..end).step_by(idx) {
                            v.clear(i - start);
                        }
                    }
                }

                v
            });
            threads.push(result);
            start = end;
        }
        for thread in threads {
            let mut v = thread.join().unwrap();
            result.append(&mut v);
        }
    }
    // let _ = result.split_off(max);
    result
}

// like p16, but working on bits instead of bytes
fn p17(max: u64, print_detail: bool) {
    let page_size_min = if print_detail { 256 } else { 4096 };
    let page_size_max = if print_detail { 128 * 1024 } else { 4096 };
    let mut page_size = page_size_min;
    let thread_min = if print_detail { 8 } else { 16 };
    let thread_max = if print_detail { 32 } else { 16 };
    let mut min_time = 99999999999;
    let mut thread_count = thread_min;
    let real_max = (max + 63) / 64 * 64;

    let _ = PackedBits::new_set(100, true);

    if print_detail {
        print!("{:7}", "thd/pg");
        while thread_count <= thread_max {
            print!("{:7}", thread_count);
            thread_count *= 2;
        }
        println!();
    }

    while page_size <= page_size_max {
        if print_detail {
            print!("{:7}", page_size);
        }
        let mut thread_count = thread_min;
        while thread_count <= thread_max {
            let time_start = std::time::SystemTime::now();
            let primes = recursive_primes_p17(real_max as usize, thread_count, page_size);
            let sum = primes.count_ones(max as usize);

            let time_elapsed = time_start.elapsed().unwrap().as_millis();
            if print_detail {
                if time_elapsed < min_time + min_time / 10 {
                    if time_elapsed < min_time {
                        min_time = time_elapsed;
                    }
                    print!("\x1b[93m");
                }
                print!("{:7}\x1b[0m", time_elapsed);
            }
            let check = |expected| {
                if sum != expected {
                    println!("\x1b[93msum={}, but should be ~{}\x1b[0m", sum, expected);
                }
            };
            match max {
                3_000_000_000 => check(144449537),
                300_000_000 => check(16252325),
                30_000_000 => check(1857859),
                3_000_000 => check(216816),
                300 => check(62),
                _ => {
                    println!(
                        "\x1b[93mhave {}, but sum unknown for {}\x1b[0m",
                        sum, real_max
                    );
                }
            }
            if !print_detail {
                // impossible
                println!(
                    "P17: Time elapsed: {}, sum: {}, max: {}M, threads: {}, page_size: {}",
                    time_elapsed,
                    sum,
                    real_max / 1_000_000,
                    thread_count,
                    page_size
                );
            }
            thread_count *= 2;
        }
        if print_detail {
            println!();
        }

        page_size *= 2;
    }
}

fn main() {
    let args = env::args();
    let num = 3_000_000_000;
    // let num = 300_000_000;
    // let num = 30_000_000;
    // let num = 3_000_000;
    // let num = 300;
    p17(num, true);
    if args.len() > 0 {
        p16(num, false);
        p12a(num);
        p12(num);
        p11(num);
        p10(num);
        p8(num);
        if num <= 3_000_000 {
            p7(num);
            p6(num);
            p5(num);
            p4(num);
            p3(num);
            p2(num);
            p1(num);
        }
    }
}
