pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut current_prime = 2;
    let mut current_number = n;

    loop {
        if current_number % current_prime == 0 {
            prime_factors.push(current_prime);
            current_number = current_number / current_prime;
            continue;
        } else if current_number == 1 {
            break;
        }
        current_prime += 1;
    }
    prime_factors
}
