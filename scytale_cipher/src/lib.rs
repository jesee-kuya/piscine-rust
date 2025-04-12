pub fn scytale_cipher(message: String, i: u32) -> String {
    let mut s = String::new();
    let mut arr = vec![];
    let mut bigarr = vec![];

    for c in message.chars() {
        arr.push(c);

        if arr.len() as u32 == i  {
            bigarr.push(arr);
            arr = vec![];
        }
    }

    if arr.len() != 0 {
        bigarr.push(arr);
    }

    let mut n = 0;

    while n < i {
        for val in &bigarr {
            if  n < val.len() as u32 {
                s.push(val[n as usize])
            } else {
                s.push(' ')
            }
        }
        n += 1
    }

    s.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(scytale_cipher(String::from("scytale Code"), 6), "sec yCtoadle".to_string());
        assert_eq!(scytale_cipher(String::from("scytale Code"), 8), "sCcoydtea l e".to_string());
    }
}
