pub fn landscape_type(nums: Vec<u8>) -> &'static str {
    let mut previus: Option<u8> = None;
    let mut directions: Vec<&str> = Vec::new();

    for current in nums {
        if previus != None {
            if Some(current) > previus {
                previus = Some(current);
                if directions.last() != Some(&"up") {
                    directions.push("up");
                }
            } else if Some(current) < previus {
                previus = Some(current);
                if directions.last() != Some(&"down") {
                    directions.push("down");
                }
            }
        } else {
            previus = Some(current);
        }
    }
    match directions.join("-").as_str() {
        "up-down" => return "mountain",
        "down-up" => return "valley",
        _ => return "neither",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01() {
        // LandscapeType([3, 4, 5, 4, 3]) ➞ "mountain"
        let result = landscape_type(vec![3, 4, 5, 4, 3]);
        assert_eq!(result, "mountain");
    }

    #[test]
    fn test02() {
        // LandscapeType([9, 7, 3, 1, 2, 4]) ➞ "valley"
        let result = landscape_type(vec![9, 7, 3, 1, 2, 4]);
        assert_eq!(result, "valley");
    }

    #[test]
    fn test03() {
        // LandscapeType([9, 8, 9]) ➞ "valley"
        let result = landscape_type(vec![9, 8, 9]);
        assert_eq!(result, "valley");
    }

    #[test]
    fn test04() {
        // LandscapeType([9, 8, 9, 8]) ➞ "neither"
        let result = landscape_type(vec![9, 8, 9, 8]);
        assert_eq!(result, "neither");
    }

    #[test]
    fn test05() {
        let result = landscape_type(vec![7, 6, 5, 4, 3]);
        assert_eq!(result, "neither");
    }

    #[test]
    fn test06() {
        let result = landscape_type(vec![7, 6, 5, 4, 5]);
        assert_eq!(result, "valley");
    }

    #[test]
    fn test07() {
        let result = landscape_type(vec![2, 3, 3, 4, 5]);
        assert_eq!(result, "neither");
    }
}
