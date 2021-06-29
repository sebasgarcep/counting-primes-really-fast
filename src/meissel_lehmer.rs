struct PrimeTable {
    limit: usize,
    primes: Vec<usize>, 
}

impl PrimeTable {
    pub fn new(limit: usize) -> PrimeTable {
        let capacity = (1.5 * (limit as f64) / (limit as f64).ln()) as usize;
        let mut primes = Vec::with_capacity(capacity);

        let mut flags = Vec::with_capacity(limit);
        for _ in 0..limit {
            flags.push(true);
        }
        flags[0] = false;

        for i in 0..limit {
            if flags[i] {
                let x = i + 1;
                for m in ((i + x)..limit).step_by(x) {
                    flags[m] = false;
                }
                primes.push(x);
            }
        }

        return PrimeTable { limit, primes };
    }

    pub fn get_prime_count(&self, n: usize) -> Option<usize> {
        if n > self.limit { return None; }
        let mut start = 0;
        let mut end = self.primes.len() - 1;
        while end - start > 1 {
            let middle = (start + end) / 2;
            if self.primes[middle] <= n {
                start = middle;
            } else {
                end = middle;
            }
        }
        if start == end || self.primes[end] > n {
            return Some(start + 1);
        } else {
            return Some(end + 1);
        }
    }

    pub fn get_prime(&self, i: usize) -> usize {
        return self.primes[i - 1];
    }
}

fn phi(n: usize, a: usize, table: &PrimeTable) -> usize {
    if a == 0 {
        return n;
    } else if a == 1 {
        return n - (n / 2);
    } else if n <= table.get_prime(a) {
        return 1;
    }
    let mut result = n - (n / 2) - (n / 3) + (n / 6);
    for i in 3..(a + 1) {
        result -= phi(n / table.get_prime(i), i - 1, &table);
    }
    return result;
}

fn integer_nth_root(n: usize, r: f64) -> usize {
    return (n as f64).powf(1.0 / r) as usize;
}

fn meissel_lehmer(n: usize, table: &PrimeTable) -> usize {
    if let Some(result) = table.get_prime_count(n) {
        return result;
    }

    let a = meissel_lehmer(integer_nth_root(n, 4.0), &table);
    let b = meissel_lehmer(integer_nth_root(n, 2.0), &table);
    let c = meissel_lehmer(integer_nth_root(n, 3.0), &table);

    let mut result = phi(n, a, &table) + ((b + a - 2) * (b - a + 1)) / 2;

    // Calculate P_2
    for i in (a + 1)..(b + 1) {
        result -= meissel_lehmer(n / table.get_prime(i), &table);
    }

    // Calculate P_3
    for i in (a + 1)..(c + 1) {
        let b_i = meissel_lehmer(integer_nth_root(n / table.get_prime(i), 2.0), &table);
        for j in i..(b_i + 1) {
            let denominator = table.get_prime(i) * table.get_prime(j);
            result -= meissel_lehmer(n / denominator, &table) - (j - 1);
        }
    }

    return result;
}

#[allow(dead_code)]
pub fn count(n: usize) -> usize {
    let limit = std::cmp::min(n, usize::pow(10, 9));
    let prime_table = PrimeTable::new(limit);
    return meissel_lehmer(n, &prime_table);
}