pub fn progress_days(days:Vec<u8>) -> u8 {
    let mut counter:usize = 0;
    let mut p_days:u8 = 0;
    
    while counter <= days.len()-1 {
        if counter > 0 {
            // days[counter-1] = Previous day.
            // days[counter]   = Actual day.
            if days[counter-1] < days[counter] {
                p_days += 1;
            }
        }
        counter += 1;        
    }
    p_days
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        assert_eq!(progress_days([3, 4, 1, 2].to_vec()), 2);
    }

    #[test]
    fn test_02() {
        assert_eq!(progress_days([10, 11, 12, 9, 10].to_vec()), 3);
    }

    #[test]
    fn test_03() {
        assert_eq!(progress_days([6, 5, 4, 3, 2, 9].to_vec()), 1);
    }

    #[test]
    fn test_04() {
        assert_eq!(progress_days([9, 9].to_vec()), 0);
    }
}
