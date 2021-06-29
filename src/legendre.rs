fn isqrt(x: usize) -> usize {
    return (x as f64).sqrt() as usize;
}

// Calculate primes up to isqrt(n)
fn get_primes(n: usize) -> Vec<usize> {
    let l = isqrt(n);
    let capacity = (1.5 * (l as f64) / (l as f64).ln()) as usize;
    let mut primes = Vec::with_capacity(capacity);
    let mut flags = Vec::with_capacity(l);
    for _ in 0..l {
        flags.push(true);
    }
    flags[0] = false;

    for i in 0..l {
        if flags[i] {
            let x = i + 1;
            for m in ((i + x)..l).step_by(x) {
                flags[m] = false;
            }
            primes.push(x);
        }
    }
    return primes;
}

fn get_max_depth(n: usize, primes: &Vec<usize>) -> usize {
    let mut max_depth = 0;
    let mut min_product = 1;
    while max_depth < primes.len() {
        min_product *= primes[max_depth];
        if min_product > n { break; }
        max_depth += 1;
    }
    return max_depth;
}

fn calculate_sum(n: usize, primes: &Vec<usize>, depth: usize, level: usize, maybe_last_index: Option<usize>, product: usize) -> usize {
    if depth < level { return n / product; }
    let mut result = 0;
    let start_index = match maybe_last_index {
        Some(last_index) => last_index + 1,
        None => 0,
    };
    let end_index = primes.len() - depth + level;
    for index in start_index..end_index {
        let next_level = level + 1;
        let next_last_index = Some(index);
        let next_product = product * primes[index];
        if next_product > n { break; }
        result += calculate_sum(n, primes, depth, next_level, next_last_index, next_product);
    }
    return result;
}

#[allow(dead_code)]
pub fn count(n: usize) -> usize {
    let mut result: isize = 0;
    let primes = get_primes(n);
    let max_depth = get_max_depth(n, &primes);
    
    // Use Legendre's Formula
    result += primes.len() as isize;
    result -= 1;

    for depth in 0..(max_depth + 1) {
        let term = calculate_sum(n, &primes, depth, 1, None, 1) as isize;
        if depth % 2 == 0 {
            result += term;
        } else {
            result -= term;
        }
    }

    return result as usize;
}