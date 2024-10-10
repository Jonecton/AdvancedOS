fn most_frequent_word(text: &str) -> (String, usize) {

    let mut words: Vec<&str> = text.split_whitespace().collect();
    words.sort();

    let mut most_common_word = "";
    let mut most_common_word_count = 0;

    let mut current_word = "";
    let mut current_count = 0;

    for &word in words.iter() {
        if word == current_word {
            current_count += 1;
        } else {
            if current_count > most_common_word_count {
                most_common_word_count = current_count;
                most_common_word = current_word;
            }

            current_word = word;
            current_count = 1;
        }
    }

    if current_count > most_common_word_count {
        most_common_word_count = current_count;
        most_common_word = current_word;
    }

    (most_common_word.to_string(), most_common_word_count)
}
fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}