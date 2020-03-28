pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut current = 2;
    loop {
        if primes.len() == (n + 1) as usize {
            break;
        }

        if is_prime(current) {
            primes.push(current);
        }
        current += 1
    }
    
    *primes.get(n as usize).unwrap()
}

fn is_prime(number: u32) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false
        };
    };
    return number > 1
}