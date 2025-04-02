pub fn is_empty(v: &str) -> bool {
    v.chars().count() == 0
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars() {
        let n = c as u32;
        if n > 177 {
            return false
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_empty(""), true);
        assert_eq!(is_ascii("rust"), true);
        assert_eq!(contains("rust", "ru"), true);
        assert_eq!(split_at("rust", 2), ("ru", "st"));
    }
}
