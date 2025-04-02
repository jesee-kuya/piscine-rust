#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Matrix (pub(i32, i32), pub(i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    Matrix((m.0.0, m.1.0), (m.0.1, m.1.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(transpose(Matrix((1, 3), (4, 5))), Matrix((1, 4), (3, 5)));
    }
}
