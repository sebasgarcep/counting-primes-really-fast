#[allow(dead_code)]
pub fn count(n: usize) -> usize {
    let mut result = 0;

    let mut flags = Vec::with_capacity(n);
    for _ in 0..n {
        flags.push(true);
    }
    flags[0] = false;

    for i in 0..n {
        if flags[i] {
            let x = i + 1;
            for m in ((i + x)..n).step_by(x) {
                flags[m] = false;
            }
            result += 1;
        }
    }

    return result;
}