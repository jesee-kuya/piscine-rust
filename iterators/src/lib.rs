#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return None;
        }
        let current = self.v;
        if current == 1 {
            self.v = 0;
            Some(current)
        } else {
            let next_v = if current % 2 == 0 {
                current / 2
            } else {
                3 * current + 1
            };
            self.v = next_v;
            Some(current)
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 || n == 1 {
        return 0;
    }
    Collatz::new(n).count() - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(collatz(0), 0);
        assert_eq!(collatz(1), 0);
        assert_eq!(collatz(4), 2);
        assert_eq!(collatz(5), 5);
        assert_eq!(collatz(6), 8);
        assert_eq!(collatz(7), 16);
        assert_eq!(collatz(12), 9);
    }
}
