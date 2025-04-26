pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    if n == 0 {
        return vec![];
    }
    
    let mut left = vec![1; n];
    let mut right = vec![1; n];
    
    for i in 1..n {
        left[i] = left[i-1] * arr[i-1];
    }
   
    for i in (0..n-1).rev() {
        right[i] = right[i+1] * arr[i+1];
    }
    
    left.iter().zip(right.iter()).map(|(l, r)| l * r).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: Vec<usize> = vec![1, 7, 3, 4];
        let output = get_products(arr);
        assert_eq!(output, vec![84, 12, 28, 21]);
    }
}
