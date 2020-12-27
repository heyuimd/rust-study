fn main() {
    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);

    println!("{}", word);

    let my_string_literal = "hello world";

    let word = first_word(my_string_literal);

    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}