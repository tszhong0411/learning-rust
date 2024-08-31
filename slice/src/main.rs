fn main() {
    let s = String::from("hello world");
    let word_index = first_word(&s);

    println!("{}", word_index);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
