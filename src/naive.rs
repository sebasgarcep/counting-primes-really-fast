fn is_prime(x: usize) -> bool {
    for d in 2..x {
        if x % d == 0 {
            return false;
        }
    }

    return true;
}

#[allow(dead_code)]
pub fn count(n: usize) -> usize {
    let mut result = 0;

    for x in 2..(n + 1) {
        if is_prime(x) {
            result += 1;
        }
    }

    return result;
}