fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; // 0 and 1 are not prime
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false; // If n is divisible by any number between 2 and sqrt(n), it's not prime
        }
        i += 1;
    }
    true // If no divisor is found, n is prime
}

fn main() {
    let num = 23; // Example number to check for primality
    if is_prime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }
}
