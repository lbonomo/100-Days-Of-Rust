use std::collections::HashMap;

pub fn sock_pairs(socks:&str) -> u8 {
    let mut pairs: HashMap<char, u8> = HashMap::new();
    
    for sock in socks.chars()  {
        match pairs.get(&sock) {
            Some(n) => pairs.insert(sock, n+1),
            None         => pairs.insert(sock, 1)
        };
    }
    
    let mut count:u8 = 0;
    for pair in pairs.values() {
        count += pair / 2
    }
    return count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        // SockPairs("AA") ➞ 1
        assert_eq!(sock_pairs("AA"), 1);
    }

    #[test]
    fn test_02() {
        // SockPairs("ABABC") ➞ 2
        assert_eq!(sock_pairs("ABABC"), 2);
    }

    #[test]
    fn test_03() {
        // SockPairs("CABBACCC") ➞ 4
        assert_eq!(sock_pairs("CABBACCC"), 4);
    }
}
