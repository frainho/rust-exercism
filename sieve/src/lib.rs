pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = vec![1; upper_bound as usize + 1];

    primes[0] = 0;
    primes[1] = 0;

    for i in 2..=upper_bound as usize {
        if primes[i] == 1 {
            for j in ((i + i)..=upper_bound as usize).step_by(i) {
                primes[j] = 0;
            }
        }
    }

    primes
        .iter()
        .enumerate()
        .filter_map(|(idx, number)| if *number == 1 { Some(idx as u64) } else { None })
        .collect()
}
