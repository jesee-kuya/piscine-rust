pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut groceries = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
        ];

        let result = vec![
            "yogurt".to_string(),
            "panettone".to_string(),
            "bread".to_string(),
            "cheese".to_string(),
            "nuts".to_string(),
        ];

        insert(&mut groceries, String::from("nuts"));
        assert_eq!(groceries, result );

        assert_eq!(at_index(&groceries, 1),groceries[1])
    }
}
