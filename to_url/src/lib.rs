pub fn to_url(s: &str) -> String {
    let mut res = String::from("");
    for c in s.chars() {
        if c == ' ' {
            res.push_str("%20");
        } else {
            res.push(c);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        assert_eq!(to_url(s), "Hello,%20world!");
    }
}
