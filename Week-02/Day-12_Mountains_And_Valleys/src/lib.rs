pub fn landscape_type(nums: Vec<i16>) -> &'static str {
    let mut previus: Option<i16> = None;
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
        assert_eq!(landscape_type(vec![3, 4, 5, 4, 3]), "mountain");
    }

    #[test]
    fn test02() {
        // LandscapeType([9, 7, 3, 1, 2, 4]) ➞ "valley"
        assert_eq!(landscape_type(vec![9, 7, 3, 1, 2, 4]), "valley");
    }

    #[test]
    fn test03() {
        // LandscapeType([9, 8, 9]) ➞ "valley"
        assert_eq!(landscape_type(vec![9, 8, 9]), "valley");
    }

    #[test]
    fn test04() {
        // LandscapeType([9, 8, 9, 8]) ➞ "neither"
        assert_eq!(landscape_type(vec![9, 8, 9, 8]), "neither");
    }

    #[test]
    fn test05() {
        assert_eq!(landscape_type(vec![7, 6, 5, 4, 3]), "neither");
    }

    #[test]
    fn test06() {
        assert_eq!(landscape_type(vec![7, 6, 5, 4, 5]), "valley");
    }

    #[test]
    fn test07() {
        assert_eq!(landscape_type(vec![2, 3, 3, 4, 5]), "neither");
    }

    #[test]
    fn test08() {
        assert_eq!(landscape_type(vec![1, 3, 5, 4, 3, 2]), "mountain");
    }

    #[test]
    fn test09() {
        assert_eq!(landscape_type(vec![-1, 0, -1]), "mountain");
    }

    #[test]
    fn test10() {
        assert_eq!(landscape_type(vec![-1, -1, 0, -1, -1]), "mountain");
    }

    #[test]
    fn test11() {
        assert_eq!(landscape_type(vec![10, 9, 8, 7, 2, 3, 4, 5]), "valley");
    }

    #[test]
    fn test12() {
        assert_eq!(landscape_type(vec![350, 100, 200, 400, 700]), "valley");
    }

    #[test]
    fn test13() {
        assert_eq!(landscape_type(vec![1, 2, 3, 2, 4, 1]), "neither");
    }
    #[test]
    fn test14() {
        assert_eq!(landscape_type(vec![10, 9, 8, 7, 2, 3, 4, 5]), "valley");
    }

    #[test]
    fn test15() {
        assert_eq!(landscape_type(vec![5, 4, 3, 2, 1]), "neither");
    }

    #[test]
    fn test16() {
        assert_eq!(landscape_type(vec![0, -1, -1, 0, -1, -1]), "neither");
    }

    #[test]
    fn test17() {
        assert_eq!(landscape_type(vec![3, 4, 5, 4, 3]), "mountain");
    }

    #[test]
    fn test18() {
        assert_eq!(landscape_type(vec![9, 7, 3, 1, 2, 4]), "valley");
    }

    #[test]
    fn test19() {
        assert_eq!(landscape_type(vec![9, 8, 9]), "valley");
    }

    #[test]
    fn test20() {
        assert_eq!(landscape_type(vec![9, 8, 9, 8]), "neither");
    }
}
