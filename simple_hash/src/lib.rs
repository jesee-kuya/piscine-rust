use std::collections::HashMap;

pub const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut hmap = HashMap::new();

    for word in words {
        *hmap.entry(word).or_insert(0) += 1;
    }
    hmap
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
        let frequency_count = word_frequency_counter(&words);


    }
}
