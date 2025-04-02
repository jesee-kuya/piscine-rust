pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = vec![];

    for word in names {
        let mut s = String::from("");

        if word.contains(".") {
            res.push(word.to_string());
            continue
        }

        let mut check = true;
        for c in word.chars() {
            if check {
                if s.len() == 0 {
                    s.push(c);
                    s.push('.');
                    check = false;
                    continue
                } else {
                    s.push(' ');
                    s.push(c);
                    s.push('.');
                    check = false;
                    continue
                }
            }
            if c == ' ' {
                check = true;
                continue
            }
        }
        res.push(s)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        assert_eq!(initials(names), vec!["H. P.".to_string(), "S. E.".to_string(), "J. L.".to_string(), "B. O.".to_string()]);
    }
}
