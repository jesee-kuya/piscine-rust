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
    let mut s = String::new();
    let mut hld = String::new();

    for c in input.chars() {
        if c == ' ' || c == '\t' {
            if hld.len() > 0 {
                s.push_str(&capitalize_first(&hld));
                s.push(c); 
                hld.clear(); 
            } else {
                s.push(c); 
            }
        } else {
            hld.push(c); 
        }
    }

    if hld.len() > 0 {
        s.push_str(&capitalize_first(&hld));
    }

    s
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
