pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let x: f64 = c.into();
    (c, x.exp(), x.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let nums : Vec<&str> = a.split_whitespace().collect();
    let mut exp = String::from("");
    for num in nums {
        let res = nbr_function(num.parse().expect("error"));
        exp.push_str(&res.1.to_string());
        exp.push_str(" ");
    }
    exp = exp.trim().to_string();
    (a, exp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = 0;
        let a = "1 2 4 5 6".to_owned();

        assert_eq!(nbr_function(c), (0, 1.0, f64::NEG_INFINITY));
        assert_eq!(str_function(a), ("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string()))
    }
}
