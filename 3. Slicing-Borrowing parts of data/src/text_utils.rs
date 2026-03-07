pub fn get_first_word(sentence: &str) -> &str {
    let first_word = sentence.split_whitespace().next().unwrap_or("");
    first_word
}

pub fn get_last_word(sentence: &str) -> &str {
    let sentence_bytes = sentence.as_bytes();
    let mut all_words: Vec<&str> = Vec::new();
    let mut start_index = 0;

    for (i, &item) in sentence_bytes.iter().enumerate() {
        if item == b' ' {
            all_words.push(&sentence[start_index..i]);
            start_index = i + 1;
        }
    }
    if start_index < sentence_bytes.len() {
        all_words.push(&sentence[start_index..]);
    }
    all_words.pop().unwrap_or("")
}
