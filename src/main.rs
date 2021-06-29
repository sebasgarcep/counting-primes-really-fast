mod naive;
mod sieve;
mod legendre;
mod meissel_lehmer;

use std::time::Instant;

fn bench(name: &str, implementation: &dyn Fn(usize) -> usize, n: usize) {
    println!("Begun testing the {} implementation with n = {}.", name, n);
    let now = Instant::now();
    let pi = implementation(n);
    println!("The amount of prime numbers that are less than or equal to {} is {}.", n, pi);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Finished testing the {} implementation with n = {}.", name, n);
    println!("-------------------------------------------------------------------");
}

fn main() {
    bench("naive", &naive::count, 100000);
    bench("sieve", &sieve::count, usize::pow(10, 9));
    bench("legendre", &legendre::count, usize::pow(10, 10));
    bench("meissel_lehmer", &meissel_lehmer::count, usize::pow(10, 10));
    bench("meissel_lehmer", &meissel_lehmer::count, usize::pow(10, 12));
}