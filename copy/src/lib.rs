pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let x: f64 = c.into();
    (c, x.exp(), x.abs().ln())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = 0;
        assert_eq!(nbr_function(c), (0, 1.0, f64::NEG_INFINITY));
    }
}
