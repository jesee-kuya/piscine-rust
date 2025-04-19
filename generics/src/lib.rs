pub fn identity<T>(v: T) -> T{
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(identity("Hello, world!"), "Hello, world!");
        assert_eq!(identity(3), 3);
    }
}
