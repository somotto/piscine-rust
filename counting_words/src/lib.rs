use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    // Convert to lowercase for case-insensitive comparison
    let lower = words.to_lowercase();

    // Use regex to match words, numbers, or apostrophes in contractions
    let re = regex::Regex::new(r"\b[a-z0-9]+(?:'[a-z0-9]+)?\b").unwrap();

    for word in re.find_iter(&lower) {
        let word = word.as_str().to_string();
        *result.entry(word).or_insert(0) += 1;
    }

    result
}
