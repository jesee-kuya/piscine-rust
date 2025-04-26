use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T: Hash + Eq, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let min_len = std::cmp::min(keys.len(), values.len());
    let mut map = HashMap::with_capacity(min_len);
    for i in 0..min_len {
        map.insert(&keys[i], &values[i]);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slices_to_map() {
        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2];
        let expected = HashMap::from([
            ("Olivia", &1),
            ("Liam", &3),
            ("Emma", &23),
            ("Noah", &5),
            ("James", &2),
        ]);
        assert_eq!(slices_to_map(&keys, &values), expected);
    }
}
