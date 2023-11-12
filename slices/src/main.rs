fn main() {
    let s1 = String::from("hello world");

    let first_word_position = first_word(&s1);
    println!("first_word_position: {first_word_position}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

