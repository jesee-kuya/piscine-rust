pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut arr = vec![];
    let mut num = String::new();

    for c in s.chars() {
        if c == 'k' {
            let mut n = num.parse::<f64>().unwrap();
            n = n * 1000.0;
            arr.push(n as u32);
            num = String::new();
        } else if c == ' ' {
            if num.len() > 0 {
                let n = num.parse::<u32>().unwrap();
                arr.push(n);
            }
            num = String::new();
        } else {
            num.push(c);
        }
    }
    if num.len() > 0 {
        let n = num.parse::<u32>().unwrap();
        arr.push(n);
    }
    Box::new(arr)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let new_str = String::from("5.5k 8.9k 32");
        let a_h = transform_and_save_on_heap(new_str);
        assert_eq!(a_h, Box::new(vec![5500, 8900, 32]));
        assert_eq!(take_value_ownership(a_h), vec![5500, 8900, 32])
    }
}
