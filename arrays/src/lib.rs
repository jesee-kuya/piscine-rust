pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thirtytwo_tens() {
        let arr = [10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];
        assert_eq!(thirtytwo_tens(), arr);
    }

    fn test_sum() {
        let a = (1..=10).collect();
        let b = [_];
        assert_eq!(sum(a), 55);
        assert_eq!(sum[b], 50);
    }
}
