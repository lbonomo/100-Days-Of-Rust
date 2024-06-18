use std::{char, collections::HashMap};


enum Option<T>{ Some(T), None }

const DIGITS:HashMap<u8, Vec<char>> = HashMap::from([
    (2, vec!['a', 'b', 'c']),
    (3, vec!['d', 'e', 'f']),
    (4, vec!['g', 'h', 'i']),
    (5, vec!['j', 'k', 'l']),
    (6, vec!['m', 'n', 'o']),
    (7, vec!['p', 'q', 'r', 's']),
    (8, vec!['t', 'u', 'v']),
    (9, vec!['w', 'x', 'y', 'z']), 
]);


pub fn add(digits:&str) -> Vec<&str> {
    // - `0 <= digits.length <= 4`
    // - `digits[i] is a digit in the range ['2', '9'].`

    if digits.len() == 0 { return vec![]; }

    // if digits.len() == 1 { return DIGITS[digits] }

    // for n in digits {
    //     print!("{}", n);
    // }
}

/**
 * 2: a b c
 * 3: d e f
 * 4: g h i
 * 5: j k l
 * 6: m n o
 * 7: p q r s
 * 8: t u v
 * 9: w x y z
 */

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_01() {
        // Input: digits = "23"
        // Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
        assert_eq!(add("23"), vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    }

    // #[test]
    // fn test_02() {
        // Input: digits = ""
        // Output: []
        // assert_eq!(add(""), vec![]);
    // }

    #[test]
    fn test_03() {
        // Input: digits = "2"
        // Output: ["a","b","c"]
        assert_eq!(add("2"), vec!["a","b","c"]);
    }    
}
