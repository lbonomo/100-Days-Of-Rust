
fn is_prime(num: u8) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..num{
        if num % i == 0 {
            return false;
        }
    }

    return true
}

pub fn next_prime(num: u8) -> u8 {
    let mut prime:u8 = num;
    while ! is_prime(prime) {
        prime += 1;
    }
    println!("{}", prime);
    return prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_01() {
        // NextPrime(12) ➞ 13
        assert_eq!(next_prime(12), 13);
    }

    #[test]
    fn it_works_02() {
        // NextPrime(24) ➞ 29
        assert_eq!(next_prime(24), 29);
    }

    #[test]
    fn it_works_03() {
        // NextPrime(11) ➞ 11
        assert_eq!(next_prime(11), 11);
    }

}
