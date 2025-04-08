use::std::fs::File;

pub fn open_file(s: &str) -> File {
    File::open(s).unwrap()
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
