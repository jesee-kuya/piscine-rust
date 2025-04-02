pub fn doubtful(s: &mut String ) {
    s.push_str("?")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = "Hello".to_owned();
        doubtful(&mut s);
        assert_eq!(s, "Hello?");
    }
}
