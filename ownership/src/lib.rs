pub fn first_subword(s: String) -> String {
    let mut res = String::from("");
    for (_, &c) in s.as_bytes().iter().enumerate() {
        if (c.is_ascii_uppercase() && res.len() > 0) || (c == b'_' && res.len() > 0) {
            break;
        }
        res.push(c as char);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "helloWorld".to_owned();
        let s2 = "snake_case".to_owned();
        let s3 = "CamelCase".to_owned();
        let s4 = "just".to_owned();

        assert_eq!(first_subword(s1), "hello".to_string());
        assert_eq!(first_subword(s2), "snake".to_string());
        assert_eq!(first_subword(s3), "Camel".to_string());
        assert_eq!(first_subword(s4), "just".to_string());
    }
}
