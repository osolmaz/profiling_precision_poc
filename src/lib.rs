use rand::rngs::ThreadRng;
use rand::Rng;

pub fn generate_random_u64(rng: &mut ThreadRng) {
    let _a = rng.gen::<u64>();
}

pub fn increment_u128(a: &mut u128) {
    *a = *a + 1;
}

// #[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn empty() {}
