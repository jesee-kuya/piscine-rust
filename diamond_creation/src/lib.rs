pub fn get_diamond(c: char) -> Vec<String> {
    let max = c as u8 - b'A';
    let mut lines = Vec::new();

    for i in 0..=max {
        let letter = (b'A' + i) as char;
        let outer_spaces = (max - i) as usize;
        let inner_spaces = if i == 0 { 0 } else { (i * 2 - 1) as usize };

        let mut line = " ".repeat(outer_spaces);
        line.push(letter);
        if inner_spaces > 0 {
            line.push_str(&" ".repeat(inner_spaces));
            line.push(letter);
        }
        line.push_str(&" ".repeat(outer_spaces));
        lines.push(line);
    }

    let mut result = lines.clone();
    for line in lines.iter().rev().skip(1) {
        result.push(line.to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_diamond('A'), ["A"]);
        assert_eq!(get_diamond('C'), ["  A  ", " B B ", "C   C", " B B ", "  A  "]);
    }
}
