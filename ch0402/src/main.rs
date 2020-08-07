// References and Borrowing
fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
    change(&mut s1);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);


    let mut s = String::from("rust");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    let r3 = &s;
    let r4 = &s;
    println!("{}, {}", r3, r4);
    let r5 = &mut s;
    println!("{}", r5);

    let r6 = no_dangle();
    println!("{}", r6);
}

fn no_dangle() -> String {
    String::from("hello rust")
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
