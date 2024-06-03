pub fn find_memo(text: &str) -> String {
    let words = text.split(' ');
    let mut counter: i8 = 1;

    for word in words {
        if word == "Nemo" {
            let text = format!("I found Nemo at {}!", counter);
            return text;
        }
        counter += 1;
    }

    return "I didn't find Nemo".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text1() {
        assert_eq!(find_memo("I am finding Nemo !"), "I found Nemo at 4!");
    }

    #[test]
    fn text2() {
        assert_eq!(find_memo("Nemo is me"), "I found Nemo at 1!")
    }
    #[test]
    fn text3() {
        assert_eq!(find_memo("I Nemo am"), "I found Nemo at 2!")
    }

    #[test]
    fn text4() {
        assert_eq!(find_memo("I'm not NeMo"), "I didn't find Nemo")
    }

    #[test]
    fn text5() {
        assert_eq!(
            find_memo("Nemo N°1 and Nemo N°2 are different"),
            "I found Nemo at 1!"
        )
    }
}
