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

pub fn do_operations(v: &mut [String]) {
    for word in v {
        let mut eq = String::new();
        let mut hld: i32 = 0;
        
        for c in word.chars() {
            if c == '+' {
                eq = "add".to_string();
                continue;
            }else if c == '-' {
                eq = "subtract".to_string();
                continue;
            }

            if eq == "add" {
                let n: i32 = c as i32 - '0' as i32;
                hld += n;
                continue;
            } else if eq == "subtract" {
                let n: i32 = c as i32 - '0' as i32;
                hld -= n;
                continue;
            }

            hld = (hld * 10) + (c as i32 - '0' as i32);
        }
        *word = hld.to_string();
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        delete_and_backspace(&mut a);
        let mut b = [
        "2+2".to_owned(),
        "3+2".to_owned(),
        "10-3".to_owned(),
        "5+5".to_owned(),
        ];
        do_operations(&mut b);

        assert_eq!(a, "borrow".to_owned());
        assert_eq!(b, ["4".to_string(), "5".to_string(), "7".to_string(), "10".to_string()]);
    }
}
