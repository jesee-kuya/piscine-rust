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
            (900, C), (900, M),
            (500, D),
            (400, C), (400, D),
            (100, C),
            (90, X), (90, C),
            (50, L),
            (40, X), (40, L),
            (10, X),
            (9, I), (9, X),
            (5, V),
            (4, I), (4, V),
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
