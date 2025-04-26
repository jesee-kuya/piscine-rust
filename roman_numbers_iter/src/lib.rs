#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

use RomanDigit::*;

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Unsupported single RomanDigit value: {}", value),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let roman_numerals = [
            (1000, M),
            (900, C),
            (900, M),
            (500, D),
            (400, C),
            (400, D),
            (100, C),
            (90, X),
            (90, C),
            (50, L),
            (40, X),
            (40, L),
            (10, X),
            (9, I),
            (9, X),
            (5, V),
            (4, I),
            (4, V),
            (1, I),
        ];

        let mut result = Vec::new();

        let mut i = 0;
        while i < roman_numerals.len() {
            let (value, digit) = roman_numerals[i];
            if i + 1 < roman_numerals.len() && roman_numerals[i].0 == roman_numerals[i + 1].0 {
                // Subtractive combination
                if num >= value {
                    result.push(roman_numerals[i].1);
                    result.push(roman_numerals[i + 1].1);
                    num -= value;
                } else {
                    i += 2;
                }
            } else {
                if num >= value {
                    result.push(digit);
                    num -= value;
                } else {
                    i += 1;
                }
            }
        }

        RomanNumber(result)
    }
}

impl RomanDigit {
    fn value(&self) -> u32 {
        match self {
            Nulla => 0,
            I => 1,
            V => 5,
            X => 10,
            L => 50,
            C => 100,
            D => 500,
            M => 1000,
        }
    }
}

impl RomanNumber {
    pub fn to_u32(&self) -> u32 {
        let mut total = 0;
        let mut prev_value = 0;
        for digit in &self.0 {
            let current_value = digit.value();
            if current_value > prev_value {
                total += current_value - 2 * prev_value;
            } else {
                total += current_value;
            }
            prev_value = current_value;
        }
        total
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let current_value = self.to_u32();
        let next_value = current_value + 1;
        let next_roman = RomanNumber::from(next_value);
        self.0 = next_roman.0.clone();
        Some(next_roman)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut number = RomanNumber::from(15);
        assert_eq!(number, RomanNumber(vec![X, V]));
        assert_eq!(number.next(), Some(RomanNumber(vec![X, V, I])));
    }
}
