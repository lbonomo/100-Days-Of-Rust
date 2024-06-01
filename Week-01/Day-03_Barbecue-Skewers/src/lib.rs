pub fn grill_analyzer(skewers: [&str; 5]) -> [u8; 2] {
    let mut vegetables: u8 = 0;
    let mut nonvegetables: u8 = 0;

    for skewer in skewers.iter() {
        match skewer.find('x') {
            Some(_index) => nonvegetables += 1,
            None => match skewer.find('o') {
                Some(_index) => vegetables += 1,
                None => {}
            },
        }
    }
    [vegetables, nonvegetables]
}

#[cfg(test)]
mod tests {
    use super::*;
    // [vegetables, nonvegetables]

    #[test]
    fn grill_01() {
        let grill = [
            "--xo--x--ox--",
            "--xx--x--xx--",
            "--oo--o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--",
        ];
        assert_eq!(grill_analyzer(grill), [1, 4]);
    }

    #[test]
    fn grill_02() {
        let grill = [
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--",
        ];
        assert_eq!(grill_analyzer(grill), [2, 3]);
    }

    #[test]
    fn grill_03() {
        let grill = [
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----",
        ];
        assert_eq!(grill_analyzer(grill), [3, 2]);
    }

    #[test]
    fn grill_04() {
        let grill = [
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----",
        ];
        assert_eq!(grill_analyzer(grill), [3, 2]);
    }

    #[test]
    fn grill_05() {
        let grill = [
            "------------",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----",
        ];
        assert_eq!(grill_analyzer(grill), [2, 2]);
    }
}
