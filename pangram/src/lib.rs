use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut map = HashMap::new();

    for c in s.to_lowercase().chars() {
        if c.is_alphabetic() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
    }

    match map.len() {
        26 => true,
        _  => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
