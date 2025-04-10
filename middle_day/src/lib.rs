use chrono::{NaiveDate, Datelike};

pub fn middle_day(year: i32) -> Option<String> {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        return None
    }
    
    let yr = NaiveDate::from_yo_opt(year, (366 + 1) / 2)
        .map(|date| date.weekday());
    Some(yr?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
