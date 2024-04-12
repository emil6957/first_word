use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a string: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a string");

    let word = first_word(&input);
    println!("The first word in your string is: {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let x = &s[0..i];
            return x;
        }
    }
    &s[..]
}
