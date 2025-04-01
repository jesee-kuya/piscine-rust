pub fn sum(left: u8, right: u8) -> u8 {
    left + right
}

pub fn diff(left: i16, right: i16) -> i16 {
    left - right
}

pub fn pro(left: i8, right: i8) -> i8 {
    left * right
}

pub fn quo(left: f64, right: f64) -> f64 {
    left / right
}

pub fn rem(left: f64, right: f64) -> f64 {
    left % right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum(2, 2);
        assert_eq!(result, 4);
        assert_eq!(sum(234, 1), 235);
        assert_eq!(8 - 8, diff(8,8));
        assert_eq!(-8 * 8, pro(-8,8));
        assert_eq!(22.0 / 2.0, quo(22.0, 2.0));
        assert_eq!(-128.23 % 2.0, rem(-128.23, 2.0));
    }
}
