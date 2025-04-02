pub fn fibonacci(n: u32) -> u32 {
    let mut fib = vec![0,1];
    
    while fib.len() <= n as usize {
        fib.push(fib[fib.len() - 1] + fib[fib.len() - 2]);
    }

    fib[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(22), 17711);
        assert_eq!(fibonacci(20), 6765);
    }
}
