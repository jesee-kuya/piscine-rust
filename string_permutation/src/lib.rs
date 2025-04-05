use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut hmap = HashMap::new();

    for c in s1.chars() {
        *hmap.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        *hmap.entry(c).or_insert(0) += 1;
    }

    hmap.values().all(|&value| value % 2 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
        let word1 = "thougth";
        assert_eq!(is_permutation(word, word1), true);
    }
}
