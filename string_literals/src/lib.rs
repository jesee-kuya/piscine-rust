pub fn is_empty(v: &str) -> bool {
    v.chars().count() == 0
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars() {
       if !c.is_ascii() {
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

pub fn find(v: &str, pat: char) -> usize {
    let mut n: usize = 0;
    for c in v.chars() {
        if c == pat {
            return n
        }
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_empty(""), true);
        assert_eq!(is_ascii("rust-"), true);
        assert_eq!(contains("rust", "ru"), true);
        assert_eq!(split_at("rust", 2), ("ru", "st"));
        assert_eq!(find("rust", 'u'), 1)
    }
}
