fn main() {
    let s = String::from("Hello World");

    let first_word_slice = first_word(&s);

    println!("first_word_slice {first_word_slice}");

    let s = String::from("hello world");

    let hello = &s[0..=4];
    let world = &s[6..11];

    println!("slices: {hello} {world}");

    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // error!

    println!("the first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
