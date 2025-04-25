pub fn first_fifty_even_square() -> Vec<i32> {
    (1..)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(50)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v1 = first_fifty_even_square();
        assert_eq!(v1.len(), 50);
    }
}
