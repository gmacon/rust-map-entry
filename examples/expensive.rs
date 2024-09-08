use std::collections::HashMap;

fn main() {
    let mut words_by_length = HashMap::new();
    for word in "Four score and seven years ago".split_ascii_whitespace() {
        words_by_length
            .entry(word.len())
            .or_insert_with(|| Vec::with_capacity(1))
            .push(word.to_owned());
    }
    println!("{words_by_length:?}");
}
