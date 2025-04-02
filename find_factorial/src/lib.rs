pub fn factorial(num: u64) -> u64 {
    let mut n = 1;
    let mut res = 1;
    while n <= num {
        res *= n;
        n += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(19), 121645100408832000);
    }
}
