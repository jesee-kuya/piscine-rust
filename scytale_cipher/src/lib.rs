pub fn scytale_cipher(message: String, mut i: u32) -> String {
    let mut s = String::new();
    let mut sz = message.chars().count();
    for c in message.chars() {
        match message.chars().nth(i.try_into().unwrap()) {
            Some(ch) => {
                s.push(c);
                s.push(ch);
                sz -= 2;
            },
            None => {
                s.push(c);
                s.push(' ');
                sz -= 1;
            }
        }
        if sz <= 0 {
            break
        }
        i += 1;
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
