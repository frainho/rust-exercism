pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();

    let mut current_prime = 2;
    let mut current_number = n;

    loop {
        if current_number % current_prime == 0 {
            prime_factors.push(current_prime);
            current_number = current_number / current_prime;
            current_prime = 2;
            continue;
        } else if current_number == 1 {
            break;
        }
        current_prime = get_next_prime(current_prime);
    }

    prime_factors
}

fn get_next_prime(mut n: u64) -> u64 {
    loop {
        n = n + 1;
        if is_prime(n) {
            break n;
        }
    }
}

fn is_prime(number: u64) -> bool {
    if number == 2 || number == 3 {
        return true;
    }

    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    let mut divisor = 6;
    while divisor * divisor - 2 * divisor + 1 <= number {
        if number % (divisor - 1) == 0 {
            return false;
        }

        if number % (divisor + 1) == 0 {
            return false;
        }

        divisor += 6;
    }

    return true;
}
