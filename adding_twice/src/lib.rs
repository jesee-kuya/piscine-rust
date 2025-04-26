pub fn add_curry(num: i32) -> impl Fn(i32) -> i32 {
    move |x| num + x
}

pub fn twice(f: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    move |x| f(f(x))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let add10 = add_curry(10);
        let value = twice(add10);
        assert_eq!(value(7), 27);
    }
}
