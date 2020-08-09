// The Slice Type
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("the first word is: {}", word);

    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("the first word is: {}", word);
    let word = first_word(my_string_literal);
    println!("the first word is: {}", word);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
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
