pub fn is_empty(v: &str) -> bool {
    v.chars().count() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_empty(""), true);
    }
}
