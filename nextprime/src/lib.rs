pub fn next_prime(nbr: u64) -> u64 {
    // Handle special cases
    if nbr <= 2 {
        return 2;
    }

    let mut candidate = if nbr % 2 == 0 { nbr + 1 } else { nbr };

    while !is_prime(candidate) {
        candidate += 2; // Only test odd numbers
    }

    candidate
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}
