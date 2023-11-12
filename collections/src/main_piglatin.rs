fn main() {
    let input_string = String::from("The quick brown fox jumps over the lazy dog");
    let input_string = input_string.to_lowercase();

    for word in input_string.split_whitespace() {
        let mut word = word.to_string();
        let first_char = word.remove(0);
        if is_vowel(&first_char) {
            let new_word = format!("{}{}-hay", first_char, word);
            println!("{}", new_word);
            continue;
        }
        let new_word = format!("{}-{}ay", word, first_char);
        println!("{}", new_word);
    }
}

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
