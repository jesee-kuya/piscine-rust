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

    s.trim().to_string()
}


pub fn change_case(input: &str) -> String {
    let mut s = String::from("");

    for c in input.chars() {
        if c >= 'a' && c <= 'z' {
            s.push((c as u8 - 32) as char);
        } else if c >= 'A' && c <= 'Z' {
            s.push((c as u8 + 32) as char);
        } else {
            s.push(c);
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("joe is missing"), "Joe is missing".to_string());
        assert_eq!(title_case("jill is leaving A"), String::from("Jill Is Leaving A"));
        assert_eq!(change_case("heLLo THere"), String::from("HEllO thERE"));
    }
}
