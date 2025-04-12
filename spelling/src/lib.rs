pub fn spell(n: u64) -> String {
    let arr1 = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let arr2 = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let arr3 = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

    match n {
        0 => "zero".to_string(),
        1..=9 => arr1[(n - 1) as usize].to_string(),
        10..=19 => arr2[(n - 10) as usize].to_string(),
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            if ones == 0 {
                arr3[(tens - 2) as usize].to_string()
            } else {
                format!("{}-{}", arr3[(tens - 2) as usize], spell(ones))
            }
        },
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} hundred", spell(hundreds))
            } else {
                format!("{} hundred {}", spell(hundreds), spell(remainder))
            }
        },
        1_000..=999_999 => {
            let thousands = n / 1_000;
            let remainder = n % 1_000;
            if remainder == 0 {
                format!("{} thousand", spell(thousands))
            } else if remainder < 100 {
                format!("{} thousand {}", spell(thousands), spell(remainder))
            } else {
                format!("{} thousand {}", spell(thousands), spell(remainder))
            }
        },
        1_000_000 => "one million".to_string(),
        _ => "number too large".to_string(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(spell(348), "three hundred forty-eight".to_string());
        assert_eq!(spell(9996), "nine thousand nine hundred ninety-six".to_string());
    }
}
