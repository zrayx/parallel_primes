#[derive(Clone)]
pub struct PackedBits {
    size: usize,
    data: Vec<u64>,
    or_table: Vec<u64>,
    and_table: Vec<u64>,
}

impl PackedBits {
    pub fn new_set(n: usize, initial_state: bool) -> PackedBits {
        let mut or_table = vec![];
        let mut and_table = vec![];
        for i in 0..64 {
            let or = 1 << i;
            let and = 0xffffffffffffffff ^ or;
            or_table.push(or);
            and_table.push(and);
        }
        let size = (n + 63) / 64;
        PackedBits {
            size: n,
            data: vec![if initial_state { 0xffffffffffffffff } else { 0 }; size],
            or_table,
            and_table,
        }
    }

    pub fn append(&mut self, other: &mut PackedBits) {
        if self.len() % 64 != 0 {
            panic!(
                "len={} and len%64={} but must be 0",
                self.len(),
                self.len() % 64
            );
        }
        self.size += other.len();
        self.data.append(&mut other.data);
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self, idx: usize) {
        let addr = idx / 64;
        let offset = idx % 64;
        let z = self.and_table[offset];
        self.data[addr] &= z;
    }

    pub fn is_set(&self, idx: usize) -> bool {
        let addr = idx / 64;
        let offset = idx % 64;
        let z = self.or_table[offset];

        self.data[addr] & z > 0
    }

    pub fn count_ones(&self, n: usize) -> usize {
        let mut sum: u32 = 0;
        let upper = n / 64;
        for d in &self.data[0..upper] {
            sum += d.count_ones();
        }

        let lower = n % 64;
        if lower > 0 {
            let r = self.data[upper];
            for i in 0..lower {
                if r & (1 << i) != 0 {
                    sum += 1;
                }
            }
        }
        sum as usize
    }
}
