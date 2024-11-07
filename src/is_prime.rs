fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    if *n == 2 {
        return true;
    }
    if *n % 2 == 0 {
        return false;
    }
    let max_check = (*n as f64).sqrt() as u32;
    let mut i = 3;

    while i <= max_check {
        if *n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn test() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    for (n, prime) in &test_data {
        assert_eq!(is_prime(n), *prime);
    }
    println!("Success!");
}