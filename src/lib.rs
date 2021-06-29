mod naive;
mod sieve;
mod legendre;
mod meissel_lehmer;

#[test]
fn test_naive() {
    assert_eq!(naive::count(1000), 168);
}

#[test]
fn test_sieve() {
    assert_eq!(sieve::count(1000), 168);
}

#[test]
fn test_legendre() {
    assert_eq!(legendre::count(1000), 168);
}

#[test]
fn test_meissel_lehmer() {
    assert_eq!(meissel_lehmer::count(1000), 168);
}

#[test]
fn test_meissel_lehmer_2() {
    assert_eq!(meissel_lehmer::count(usize::pow(10, 12)), 37607912018);
}