pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(f_char) => {
            f_char.to_uppercase().chain(chars).collect()
        }
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    let words = input.split_whitespace();
    let mut s = String::from("");

    for word in words {
        s.push_str(&capitalize_first(word));
        s.push(' ');
    }

    s = s.trim().to_string();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("joe is missing"), "Joe is missing".to_string());
        assert_eq!(title_case("jill is leaving A"), String::from("Jill Is Leaving A"));
    }
}
