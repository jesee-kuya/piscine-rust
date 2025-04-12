pub fn pig_latin(text: &str) -> String {
    let mut end = String::new();
    let mut s = String::new();
    let mut check = true;
    for c in text.chars() {
        if !is_vowel(c) && check {
            end.push(c)
        } else {
            check = false;
            s.push(c)
        }
    }
    s.push_str(&end);
    s.push_str("ay");
    s
}

pub fn is_vowel(c: char) -> bool {
    for vowel in ['a', 'e' , 'i', 'o', 'u'] {
        if c == vowel {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(pig_latin(&String::from("igloo")), "iglooay".to_string());
        assert_eq!(pig_latin(&String::from("apple")), "appleay".to_string());
        assert_eq!(pig_latin(&String::from("hello")), "ellohay".to_string());
    }
}
