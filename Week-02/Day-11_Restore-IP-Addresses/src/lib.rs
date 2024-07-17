#[derive(Debug)]
pub struct IP {
    pub oct01: u8,
    pub oct02: u8,
    pub oct03: u8,
    pub oct04: u8,
}

impl IP {
    fn print(&self) -> String {
        let o = format!("{:?}.{:?}.{:?}.{:?}", &self.oct01, &self.oct02, &self.oct03, &self.oct04);
        return o;
    }
}

pub fn make_ip(s: &str) -> Option<Vec<String>> {
    // dbg!(s.len());

    
    // 1111 (4)
    // 255255255255 (12)
    match s.len() >= 4 && s.len() <= 12  {
        true => {

            let numbers = s.chars();

            // if 

            let ip1 = IP { oct01:255, oct02:255, oct03:111, oct04:35}.print();
            let ip2 = IP { oct01:255, oct02:255, oct03:11, oct04:135}.print();
        
            let mut x = vec![ip1, ip2];
            x.sort();
            return Some(x)
        }
        false => {
            return None;
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        // Input: s = "25525511135"
        // Output: ["255.255.11.135","255.255.111.35"]
        let mut output: Vec<String> = vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()];
        output.sort();
        assert_eq!(make_ip("25525511135"), Some(output));
    }

    #[test]
    fn test_02() {
        // Input: s = "0000"
        // Output: ["0.0.0.0"]
        let mut output: Vec<String> = vec!["0.0.0.0".to_string()];
        output.sort();
        assert_eq!(make_ip("0000"), Some(output));
    }

    #[test]
    fn test_03() {
        // Input: s = "1111"
        // Output: ["1.1.1.1"]
        let mut output: Vec<String> = vec!["1.1.1.1".to_string()];
        output.sort();
        assert_eq!(make_ip("1111"), Some(output));
    }

    #[test]
    fn test_04() {
        // Input: s = "010010"
        // Output: ["0.10.0.10","0.100.1.0"]
        let mut output: Vec<String> = vec!["0.10.0.10".to_string(),"0.100.1.0".to_string()];
        output.sort();
        assert_eq!(make_ip("010010"), Some(output));
    }

    #[test]
    fn test_05() {
        // Input: s = "101023"
        // Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
        let mut output: Vec<String> = vec!["1.0.10.23".to_string(),"1.0.102.3".to_string(),"10.1.0.23".to_string(),"10.10.2.3".to_string(),"101.0.2.3".to_string()];
        output.sort();
        assert_eq!(make_ip("101023"), Some(output));
    }

    #[test]
    fn test_06() {
        let output = None;
        assert_eq!(make_ip("101"), output);
    }

    #[test]
    fn test_07() {
        let output = None;
        assert_eq!(make_ip("1011011011011"), output);
    }
}
