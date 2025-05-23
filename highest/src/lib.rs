#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted = self.numbers.to_vec();
        sorted.sort_by(|a, b| b.cmp(a));
        sorted.truncate(3);
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected = [30, 500, 20, 70];
        let n = Numbers::new(&expected);
        assert_eq!(n.list(), [30, 500, 20, 70]);
        assert_eq!(n.highest(), Some(500));
        assert_eq!(n.latest(), Some(70));
        assert_eq!(n.highest_three(), vec![500, 70, 30]);
    }
}
 
