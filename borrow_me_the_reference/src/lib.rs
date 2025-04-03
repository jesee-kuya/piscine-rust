pub fn delete_and_backspace(s: &mut String) {
    let mut count = 0;
    let mut result = String::new(); 

    for c in s.chars() {
        if c == '-' {
            if result.len() >= 2 {
                result.truncate(result.len() - 1);
            }
            continue;
        } else if c == '+' {
            count += 1;
            continue;
        }

        if count > 0 {
            count -= 1;
            continue;
        }

        result.push(c);
    }

    *s = result;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        delete_and_backspace(&mut a);

        assert_eq!(a, "borrow".to_owned());
    }
}
